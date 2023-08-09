use std::fmt;
use std::io::Cursor;

use log::{debug, error};
use neli::consts::genl::{CtrlAttr, CtrlCmd};
use neli::consts::nl::{GenlId, NlType, NlmF, NlmFFlags, Nlmsg};
use neli::consts::socket::NlFamily;
use neli::err::{NlError, SerError};
use neli::genl::{Genlmsghdr, Nlattr};
use neli::nl::{NlPayload, Nlmsghdr};
use neli::socket::NlSocketHandle;
use neli::types::{Buffer, GenlBuffer};
use neli::ToBytes;

use crate::attributes::MonitorFlags;
use crate::reg_domain::RegulatoryDomain;
use crate::station::WirelessStation;
use crate::wiphy::PhysicalDevice;
use crate::{ChannelWidth, InterfaceType};

use super::attributes::Attribute;
use super::attributes::ChannelWidth as NlChannelWidth;
use super::attributes::InterfaceType as NlInterfaceType;
use super::commands::Command;
use super::interface::WirelessInterface;

const NL80211_VERSION: u8 = 1;
type Neli80211Header = Genlmsghdr<Command, Attribute>;

/// Netlink socket.
pub struct NlSocket {
    socket: NlSocketHandle,
    nl_type: u16,
}

impl NlSocket {
    /// Connect netlink socket.
    pub fn connect() -> Result<Self, NlError<GenlId, Genlmsghdr<CtrlCmd, CtrlAttr>>> {
        let mut socket = NlSocketHandle::connect(NlFamily::Generic, None, &[])?;
        let nl_type = socket.resolve_genl_family("nl80211")?;
        Ok(Self { socket, nl_type })
    }

    pub fn list_interfaces(&mut self) -> Result<Vec<WirelessInterface>, NlError> {
        let nl_payload = Genlmsghdr::<Command, Attribute>::new(
            Command::GetInterface,
            NL80211_VERSION,
            GenlBuffer::new(),
        );

        let msg = self.build_header(nl_payload, &[NlmF::Request, NlmF::Dump]);
        self.send(msg)?;

        let mut responses = Vec::new();
        for response in self.socket.iter::<Nlmsg, Neli80211Header>(false) {
            let response = response.map_err(NlError::new)?;
            match response.nl_payload {
                NlPayload::Err(e) => {
                    error!("Error when reading GetInterface response: {e}");
                    break;
                }
                NlPayload::Payload(payload) => {
                    let handle = payload.get_attr_handle();
                    responses.push(handle.try_into()?);
                }
                NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(responses)
    }

    pub fn set_interface(&mut self, if_index: u32, if_type: InterfaceType) -> Result<(), NlError> {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            attrs.push(Nlattr::new(false, false, Attribute::Ifindex, if_index).unwrap());
            attrs.push(
                Nlattr::new(
                    false,
                    false,
                    Attribute::Iftype,
                    Into::<NlInterfaceType>::into(if_type),
                )
                .unwrap(),
            );
            attrs
        };
        self.send_set_interface(attrs)
    }

    pub fn set_monitor_flags(
        &mut self,
        if_index: u32,
        flags: Vec<MonitorFlags>,
    ) -> Result<(), NlError> {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            attrs.push(Nlattr::new(false, false, Attribute::Ifindex, if_index).unwrap());
            attrs.push(
                Nlattr::new(false, false, Attribute::Iftype, NlInterfaceType::Monitor).unwrap(),
            );
            attrs.push(Nlattr::new(false, false, Attribute::MntrFlags, flags).unwrap());
            attrs
        };
        self.send_set_interface(attrs)
    }

    fn send_set_interface(&mut self, attrs: GenlBuffer<Attribute, Buffer>) -> Result<(), NlError> {
        let nl_payload =
            Genlmsghdr::<Command, Attribute>::new(Command::SetInterface, NL80211_VERSION, attrs);
        let msg = self.build_header(nl_payload, &[NlmF::Request, NlmF::Ack]);

        self.send(msg)?;
        for response in self.socket.iter::<Nlmsg, Neli80211Header>(false) {
            let response = response.map_err(NlError::new)?;
            match response.nl_payload {
                NlPayload::Err(e) => {
                    error!("Error when reading response: {e}");
                    break;
                }
                NlPayload::Payload(_) | NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(())
    }

    pub fn set_channel(
        &mut self,
        if_index: u32,
        freq: u32,
        width: ChannelWidth,
    ) -> Result<(), NlError> {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            attrs.push(Nlattr::new(false, false, Attribute::Ifindex, if_index).unwrap());
            attrs.push(Nlattr::new(false, false, Attribute::WiphyFreq, freq).unwrap());
            attrs.push(
                Nlattr::new(
                    false,
                    false,
                    Attribute::ChannelWidth,
                    Into::<NlChannelWidth>::into(width),
                )
                .unwrap(),
            );
            attrs
        };
        let nl_payload =
            Genlmsghdr::<Command, Attribute>::new(Command::SetChannel, NL80211_VERSION, attrs);
        let msg = self.build_header(nl_payload, &[NlmF::Request, NlmF::Ack]);

        self.send(msg)?;
        for response in self.socket.iter::<Nlmsg, Neli80211Header>(false) {
            let response = response.map_err(NlError::new)?;
            match response.nl_payload {
                NlPayload::Err(e) => {
                    error!("Error when reading response: {e}");
                    break;
                }
                NlPayload::Payload(_) | NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(())
    }

    pub fn list_stations(&mut self, if_index: u32) -> Result<Vec<WirelessStation>, NlError> {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            attrs.push(Nlattr::new(false, false, Attribute::Ifindex, if_index).unwrap());
            attrs
        };

        let nl_payload =
            Genlmsghdr::<Command, Attribute>::new(Command::GetStation, NL80211_VERSION, attrs);

        let msg = self.build_header(nl_payload, &[NlmF::Request, NlmF::Dump]);

        self.send(msg)?;
        let mut responses = Vec::new();
        for response in self.socket.iter::<Nlmsg, Neli80211Header>(false) {
            let response = response.map_err(NlError::new)?;
            match response.nl_payload {
                NlPayload::Err(e) => {
                    error!("Error when reading GetStation response: {e}");
                    break;
                }
                NlPayload::Payload(payload) => {
                    let handle = payload.get_attr_handle();
                    responses.push(handle.try_into()?);
                }
                NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(responses)
    }

    pub fn list_physical_devices(&mut self) -> Result<Vec<PhysicalDevice>, NlError> {
        let nl_payload = Genlmsghdr::<Command, Attribute>::new(
            Command::GetWiphy,
            NL80211_VERSION,
            GenlBuffer::new(),
        );
        let msg = self.build_header(nl_payload, &[NlmF::Request, NlmF::Dump]);
        self.send(msg)?;
        let mut responses = Vec::new();
        for response in self.socket.iter::<Nlmsg, Neli80211Header>(false) {
            let response = response.map_err(NlError::new)?;
            match response.nl_payload {
                NlPayload::Err(e) => {
                    error!("Error when reading GetWiphy response: {e}");
                    break;
                }
                NlPayload::Payload(payload) => {
                    let handle = payload.get_attr_handle();
                    responses.push(handle.try_into()?);
                }
                NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(responses)
    }

    pub fn get_regulatory_domain(&mut self) -> Result<Vec<RegulatoryDomain>, NlError> {
        let nl_payload = Genlmsghdr::<Command, Attribute>::new(
            Command::GetReg,
            NL80211_VERSION,
            GenlBuffer::new(),
        );

        let msg = self.build_header(nl_payload, &[NlmF::Request, NlmF::Dump]);

        self.send(msg)?;
        let mut responses = Vec::new();
        for response in self.socket.iter::<Nlmsg, Neli80211Header>(false) {
            let response = response.map_err(NlError::new)?;
            match response.nl_payload {
                NlPayload::Err(e) => {
                    error!("Error when reading GetReg response: {e}");
                    break;
                }
                NlPayload::Payload(payload) => {
                    let handle = payload.get_attr_handle();
                    responses.push(handle.try_into()?);
                }
                NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(responses)
    }

    fn send<T, P>(&mut self, msg: Nlmsghdr<T, P>) -> Result<(), SerError>
    where
        T: NlType + fmt::Debug,
        P: ToBytes + fmt::Debug,
    {
        if cfg!(debug_assertions) {
            let mut b: Cursor<Vec<u8>> = Cursor::new(vec![0; 15]);
            msg.nl_payload.to_bytes(&mut b).unwrap();
            let octets: String = b.get_ref().iter().map(|v| format!("{:02x} ", v)).collect();
            debug!("[PAYLOAD] {octets}");
        }
        self.socket.send(msg)
    }

    fn build_header<P: neli::Size>(&self, nl_payload: P, flags: &[NlmF]) -> Nlmsghdr<u16, P> {
        let len = None;
        let nl_type = self.nl_type;
        let flags = NlmFFlags::new(flags);
        let seq = None;
        let pid = None;
        let payload = NlPayload::Payload(nl_payload);
        Nlmsghdr::new(len, nl_type, flags, seq, pid, payload)
    }
}
