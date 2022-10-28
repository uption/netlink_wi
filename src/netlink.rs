use neli::consts::genl::{CtrlAttr, CtrlCmd};
use neli::consts::nl::{GenlId, NlmF, NlmFFlags, Nlmsg};
use neli::consts::socket::NlFamily;
use neli::err::NlError;
use neli::genl::{Genlmsghdr, Nlattr};
use neli::nl::{NlPayload, Nlmsghdr};
use neli::socket::NlSocketHandle;
use neli::types::GenlBuffer;

use crate::station::WirelessStation;
use crate::wiphy::PhysicalDevice;

use super::attributes::Attribute;
use super::commands::Command;
use super::interface::WirelessInterface;

const NL80211_VERSION: u8 = 1;
type Neli80211Header = Genlmsghdr<Command, Attribute>;

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
        self.socket.send(msg)?;

        let mut responses = Vec::new();
        for response in self.socket.iter::<Nlmsg, Neli80211Header>(false) {
            println!("{response:?}");
            let response = response.map_err(NlError::new)?;
            match response.nl_payload {
                NlPayload::Err(e) => {
                    println!("Error when reading GetInterface response: {e}");
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

    pub fn list_stations(&mut self, if_index: u32) -> Result<Vec<WirelessStation>, NlError> {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            attrs.push(Nlattr::new(false, false, Attribute::Ifindex, if_index).unwrap());
            attrs
        };

        let nl_payload =
            Genlmsghdr::<Command, Attribute>::new(Command::GetStation, NL80211_VERSION, attrs);

        let msg = self.build_header(nl_payload, &[NlmF::Request, NlmF::Dump]);

        self.socket.send(msg)?;
        let mut responses = Vec::new();
        for response in self.socket.iter::<Nlmsg, Neli80211Header>(false) {
            let response = response.map_err(NlError::new)?;
            match response.nl_payload {
                NlPayload::Err(e) => {
                    println!("Error when reading GetStation response: {e}");
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
        self.socket.send(msg)?;
        let mut responses = Vec::new();
        for response in self.socket.iter::<Nlmsg, Neli80211Header>(false) {
            let response = response.map_err(NlError::new)?;
            match response.nl_payload {
                NlPayload::Err(e) => {
                    println!("Error when reading GetWiphy response: {e}");
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
