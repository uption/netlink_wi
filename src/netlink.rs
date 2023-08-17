use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;

use log::{debug, error};
use neli::consts::nl::{NlmF, Nlmsg};
use neli::consts::socket::NlFamily;
use neli::err::RouterError;
use neli::genl::{AttrTypeBuilder, Genlmsghdr, GenlmsghdrBuilder, NlattrBuilder, NoUserHeader};
use neli::nl::NlPayload;
use neli::router::synchronous::{NlRouter, NlRouterReceiverHandle};
use neli::types::GenlBuffer;
use neli::utils::Groups;
use neli::{Size, ToBytes};

use crate::attributes::MonitorFlags;
use crate::interface::{ChannelWidth, InterfaceType};
use crate::reg_domain::RegulatoryDomain;
use crate::station::WirelessStation;
use crate::wiphy::PhysicalDevice;

use super::attributes::Attribute;
use super::attributes::ChannelWidth as NlChannelWidth;
use super::attributes::InterfaceType as NlInterfaceType;
use super::commands::Command;
use super::interface::WirelessInterface;

const NL80211_VERSION: u8 = 1;
type Neli80211Header = Genlmsghdr<Command, Attribute>;

/// Netlink socket.
pub struct NlSocket {
    socket: NlRouter,
    nl_type: u16,
}

impl NlSocket {
    /// Connect netlink socket.
    pub fn connect() -> Result<Self, Box<dyn Error>> {
        let (socket, _) = NlRouter::connect(NlFamily::Generic, None, Groups::empty())?;
        let nl_type = socket.resolve_genl_family("nl80211")?;
        Ok(Self { socket, nl_type })
    }

    pub fn list_interfaces(&mut self) -> Result<Vec<WirelessInterface>, Box<dyn Error>> {
        let nl_payload = NlPayload::Payload(
            GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                .cmd(Command::GetInterface)
                .version(NL80211_VERSION)
                .build()?,
        );
        let recv: NlRouterReceiverHandle<Nlmsg, Neli80211Header> =
            self.send(NlmF::REQUEST | NlmF::DUMP, nl_payload)?;

        let mut responses = Vec::new();
        for response in recv {
            let response = response?;
            match response.nl_payload() {
                NlPayload::Err(e) => {
                    error!("Error when reading GetInterface response: {e}");
                    break;
                }
                NlPayload::Payload(payload) => {
                    let handle = payload.attrs().get_attr_handle();
                    responses.push(handle.try_into()?);
                }
                NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(responses)
    }

    pub fn set_interface(
        &mut self,
        if_index: u32,
        if_type: InterfaceType,
    ) -> Result<(), Box<dyn Error>> {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Ifindex)
                .build()?;
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(if_index)
                    .build()?,
            );
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Iftype)
                .build()?;
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(Into::<NlInterfaceType>::into(if_type))
                    .build()?,
            );
            attrs
        };
        let nl_payload = NlPayload::Payload(
            GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                .cmd(Command::SetInterface)
                .version(NL80211_VERSION)
                .attrs(attrs)
                .build()?,
        );

        let recv: NlRouterReceiverHandle<Nlmsg, Neli80211Header> =
            self.send(NlmF::REQUEST | NlmF::ACK, nl_payload)?;

        for response in recv {
            let response = response?;
            match response.nl_payload() {
                NlPayload::Err(e) => {
                    error!("Error when reading SetInterface response: {e}");
                    break;
                }
                NlPayload::Payload(_) | NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(())
    }

    pub fn set_monitor_flags(
        &mut self,
        if_index: u32,
        flags: Vec<MonitorFlags>,
    ) -> Result<(), Box<dyn Error>> {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Ifindex)
                .build()?;
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(if_index)
                    .build()?,
            );
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Iftype)
                .build()?;
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(Into::<NlInterfaceType>::into(NlInterfaceType::Monitor))
                    .build()?,
            );
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::MntrFlags)
                .build()?;
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(flags)
                    .build()?,
            );
            attrs
        };
        let nl_payload = NlPayload::Payload(
            GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                .cmd(Command::SetInterface)
                .version(NL80211_VERSION)
                .attrs(attrs)
                .build()?,
        );

        let recv: NlRouterReceiverHandle<Nlmsg, Neli80211Header> =
            self.send(NlmF::REQUEST | NlmF::ACK, nl_payload)?;

        for response in recv {
            let response = response?;
            match response.nl_payload() {
                NlPayload::Err(e) => {
                    error!("Error when reading SetInterface response: {e}");
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
    ) -> Result<(), Box<dyn Error>> {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Ifindex)
                .build()?;
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(if_index)
                    .build()?,
            );
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::WiphyFreq)
                .build()?;
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(freq)
                    .build()?,
            );
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::ChannelWidth)
                .build()?;
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(Into::<NlChannelWidth>::into(width))
                    .build()?,
            );
            attrs
        };
        let nl_payload = NlPayload::Payload(
            GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                .cmd(Command::SetChannel)
                .version(NL80211_VERSION)
                .attrs(attrs)
                .build()?,
        );
        let recv: NlRouterReceiverHandle<Nlmsg, Neli80211Header> =
            self.send(NlmF::REQUEST | NlmF::ACK, nl_payload)?;

        for response in recv {
            let response = response?;
            match response.nl_payload() {
                NlPayload::Err(e) => {
                    error!("Error when reading SetChannel response: {e}");
                    break;
                }
                NlPayload::Payload(_) | NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(())
    }

    pub fn list_stations(&mut self, if_index: u32) -> Result<Vec<WirelessStation>, Box<dyn Error>> {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Ifindex)
                .build()?;
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(if_index)
                    .build()?,
            );
            attrs
        };

        let nl_payload = NlPayload::Payload(
            GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                .cmd(Command::GetStation)
                .version(NL80211_VERSION)
                .attrs(attrs)
                .build()?,
        );

        let recv: NlRouterReceiverHandle<Nlmsg, Neli80211Header> =
            self.send(NlmF::REQUEST | NlmF::DUMP, nl_payload)?;
        let mut responses = Vec::new();
        for response in recv {
            let response = response?;
            match response.nl_payload() {
                NlPayload::Err(e) => {
                    error!("Error when reading GetStation response: {e}");
                    break;
                }
                NlPayload::Payload(payload) => {
                    let handle = payload.attrs().get_attr_handle();
                    responses.push(handle.try_into()?);
                }
                NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(responses)
    }

    pub fn list_physical_devices(&mut self) -> Result<Vec<PhysicalDevice>, Box<dyn Error>> {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::SplitWiphyDump)
                .build()?;
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(())
                    .build()?,
            );
            attrs
        };
        let nl_payload = NlPayload::Payload(
            GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                .cmd(Command::GetWiphy)
                .version(NL80211_VERSION)
                .attrs(attrs)
                .build()?,
        );
        let recv: NlRouterReceiverHandle<Nlmsg, Neli80211Header> =
            self.send(NlmF::REQUEST | NlmF::DUMP, nl_payload)?;
        let mut responses = HashMap::new();
        for response in recv {
            let response = response?;
            match response.nl_payload() {
                NlPayload::Err(e) => {
                    error!("Error when reading GetWiphy response: {e}");
                    break;
                }
                NlPayload::Payload(payload) => {
                    let handle = payload.attrs().get_attr_handle();
                    let device: PhysicalDevice = handle.try_into()?;
                    responses
                        .entry(device.wiphy_index)
                        .and_modify(|d: &mut PhysicalDevice| d.merge(&device))
                        .or_insert(device);
                }
                NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(responses.values().cloned().collect())
    }

    pub fn get_physical_device(
        &mut self,
        wiphy_index: u32,
    ) -> Result<Option<PhysicalDevice>, Box<dyn Error>> {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Wiphy)
                .build()?;
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(wiphy_index)
                    .build()?,
            );
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::SplitWiphyDump)
                .build()?;
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(())
                    .build()?,
            );
            attrs
        };
        let nl_payload = NlPayload::Payload(
            GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                .cmd(Command::GetWiphy)
                .version(NL80211_VERSION)
                .attrs(attrs)
                .build()?,
        );
        let recv: NlRouterReceiverHandle<Nlmsg, Neli80211Header> =
            self.send(NlmF::REQUEST | NlmF::DUMP, nl_payload)?;
        let mut result: Option<PhysicalDevice> = None;
        for response in recv {
            let response = response?;
            match response.nl_payload() {
                NlPayload::Err(e) => {
                    error!("Error when reading GetWiphy response: {e}");
                    break;
                }
                NlPayload::Payload(payload) => {
                    let handle = payload.attrs().get_attr_handle();
                    let device: PhysicalDevice = handle.try_into()?;
                    if device.wiphy_index == wiphy_index {
                        if let Some(d) = result.as_mut() {
                            d.merge(&device);
                        } else {
                            result = Some(device);
                        }
                    }
                }
                NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(result)
    }

    pub fn get_regulatory_domain(&mut self) -> Result<Vec<RegulatoryDomain>, Box<dyn Error>> {
        let nl_payload = NlPayload::Payload(
            GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                .cmd(Command::GetReg)
                .version(NL80211_VERSION)
                .build()?,
        );

        let recv: NlRouterReceiverHandle<Nlmsg, Neli80211Header> =
            self.send(NlmF::REQUEST | NlmF::DUMP, nl_payload)?;

        let mut responses = Vec::new();
        for response in recv {
            let response = response?;
            match response.nl_payload() {
                NlPayload::Err(e) => {
                    error!("Error when reading GetReg response: {e}");
                    break;
                }
                NlPayload::Payload(payload) => {
                    let handle = payload.attrs().get_attr_handle();
                    responses.push(handle.try_into()?);
                }
                NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(responses)
    }

    pub fn send<SP, RT, RP>(
        &self,
        nl_flags: NlmF,
        nl_payload: NlPayload<u16, SP>,
    ) -> Result<NlRouterReceiverHandle<RT, RP>, RouterError<u16, SP>>
    where
        SP: Size + ToBytes,
    {
        if cfg!(debug_assertions) {
            let mut b: Cursor<Vec<u8>> = Cursor::new(Vec::new());
            nl_payload.to_bytes(&mut b).unwrap();
            let octets: String = b.get_ref().iter().map(|v| format!("{:02x} ", v)).collect();
            debug!("[PAYLOAD] {octets}");
        }
        self.socket.send(self.nl_type, nl_flags, nl_payload)
    }
}
