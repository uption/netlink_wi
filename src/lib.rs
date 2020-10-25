pub mod nl80211;

use neli::consts::{NlFamily, NlmF, Nlmsg};
use neli::genl::Genlmsghdr;
use neli::nl::Nlmsghdr;
use neli::nlattr::{AttrHandle, Nlattr};
use neli::socket::NlSocket as NeliSocket;

pub const NL80211_VERSION: u8 = 1;
type Neli80211Header = Genlmsghdr<nl80211::Command, nl80211::Attribute>;

/// Netlink socket for starting communication with kernel.
pub struct NlSocket {
    socket: NeliSocket,
    nl_type: u16,
}

impl NlSocket {
    /// Connect netlink socket.
    pub fn connect() -> Self {
        let mut socket = NeliSocket::connect(NlFamily::Generic, None, None, true).unwrap();
        let nl_type = socket.resolve_genl_family("nl80211").unwrap();
        Self { socket, nl_type }
    }

    fn send(&mut self, nl_payload: NlPayload) {
        self.socket.send_nl(nl_payload.into()).unwrap();
    }
}

/// Builder for 802.11 netlink header and payload.
struct Nl80211Payload {
    command: nl80211::Command,
    attributes: Vec<(nl80211::Attribute, Vec<u8>)>,
}

impl Nl80211Payload {
    fn new(command: nl80211::Command) -> Self {
        Self {
            command,
            attributes: Vec::new(),
        }
    }

    fn add_attribute(self, nla_type: nl80211::Attribute, payload: Vec<u8>) -> Self {
        let mut attributes = self.attributes.clone();
        attributes.push((nla_type, payload));
        Self {
            command: self.command,
            attributes,
        }
    }
}

impl Into<Neli80211Header> for Nl80211Payload {
    fn into(self) -> Neli80211Header {
        let attrs = self
            .attributes
            .into_iter()
            .map(|(nla_type, payload)| Nlattr::new(None, nla_type, payload).unwrap())
            .collect();
        Genlmsghdr::new(self.command, NL80211_VERSION, attrs).unwrap()
    }
}

/// Builder for netlink header and payload.
struct NlPayload {
    nl_type: u16,
    nl_flags: Vec<NlmF>,
    nl_payload: Nl80211Payload,
}

impl NlPayload {
    fn new(nl_type: u16, nl_payload: Nl80211Payload) -> Self {
        Self {
            nl_type,
            nl_flags: Vec::new(),
            nl_payload,
        }
    }

    fn add_flag(self, flag: NlmF) -> Self {
        let mut nl_flags = self.nl_flags.clone();
        nl_flags.push(flag);
        Self {
            nl_type: self.nl_type,
            nl_flags,
            nl_payload: self.nl_payload,
        }
    }
}

impl Into<Nlmsghdr<u16, Neli80211Header>> for NlPayload {
    fn into(self) -> Nlmsghdr<u16, Neli80211Header> {
        Nlmsghdr::new(
            None,
            self.nl_type,
            self.nl_flags,
            None,
            None,
            self.nl_payload.into(),
        )
    }
}

#[derive(Debug, Clone, Default)]
/// Interface information returned from netlink.
pub struct WirelessInterface {
    pub essid: Option<String>,
}

impl WirelessInterface {
    pub fn list_interfaces(socket: &mut NlSocket) -> Vec<WirelessInterface> {
        let nl_payload = Nl80211Payload::new(nl80211::Command::GetInterface);
        let msg = NlPayload::new(socket.nl_type, nl_payload)
            .add_flag(NlmF::Request)
            .add_flag(NlmF::Dump);
        socket.send(msg);

        let mut interfaces = Vec::new();
        for response in socket.socket.iter::<Nlmsg, Neli80211Header>() {
            let response = response.unwrap();
            match response.nl_type {
                Nlmsg::Noop => (),
                Nlmsg::Overrun => (),
                Nlmsg::Done => break,
                Nlmsg::Error => {
                    println!("Kernel returned an error");
                    break;
                }
                Nlmsg::UnrecognizedVariant(nl_type) if nl_type == socket.nl_type => {
                    let handle = response.nl_payload.get_attr_handle();
                    interfaces.push(handle.into());
                }
                Nlmsg::UnrecognizedVariant(nl_type) => println!("Unrecognized nl_type {}", nl_type),
            };
        }

        interfaces
    }
}

impl From<AttrHandle<'_, nl80211::Attribute>> for WirelessInterface {
    fn from(handle: AttrHandle<nl80211::Attribute>) -> Self {
        let mut interface = WirelessInterface::default();
        for attr in handle.iter() {
            match attr.nla_type {
                nl80211::Attribute::Ssid => {
                    interface.essid = Some(String::from_utf8_lossy(&attr.payload).to_string());
                }
                _ => (),
            }
        }
        interface
    }
}
