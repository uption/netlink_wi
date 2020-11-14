use std::cell::RefCell;

use neli::consts::{NlFamily, NlmF, Nlmsg};
use neli::genl::Genlmsghdr;
use neli::nl::Nlmsghdr;
use neli::nlattr::{AttrHandle, Nlattr};
use neli::socket::NlSocket as NeliSocket;

use super::attributes::Attribute;
use super::commands::Command;
use super::interface::WirelessInterface;

const NL80211_VERSION: u8 = 1;
type Neli80211Header = Genlmsghdr<Command, Attribute>;

pub struct NlSocket {
    socket: RefCell<NeliSocket>,
    nl_type: u16,
}

impl NlSocket {
    /// Connect netlink socket.
    pub fn connect() -> Self {
        let mut socket = NeliSocket::connect(NlFamily::Generic, None, None, true).unwrap();
        let nl_type = socket.resolve_genl_family("nl80211").unwrap();
        Self {
            socket: RefCell::new(socket),
            nl_type,
        }
    }

    pub fn list_interfaces(&self) -> Vec<WirelessInterface> {
        let nl_payload = Nl80211HeaderBuilder::new(Command::GetInterface);
        let msg = NetlinkHeaderBuilder::new(self.nl_type, nl_payload)
            .add_flag(NlmF::Request)
            .add_flag(NlmF::Dump);
        self.send(msg);
        self.read::<WirelessInterface>()
    }

    fn send(&self, payload: NetlinkHeaderBuilder) {
        self.socket.borrow_mut().send_nl(payload.into()).unwrap();
    }

    fn read<T: Parser>(&self) -> Vec<T> {
        let mut responses = Vec::new();
        for response in self.socket.borrow_mut().iter::<Nlmsg, Neli80211Header>() {
            let response = response.unwrap();
            match response.nl_type {
                Nlmsg::Noop => (),
                Nlmsg::Overrun => (),
                Nlmsg::Done => break,
                Nlmsg::Error => {
                    println!("Kernel returned an error");
                    break;
                }
                Nlmsg::UnrecognizedVariant(nl_type) if nl_type == self.nl_type => {
                    let handle = response.nl_payload.get_attr_handle();
                    responses.push(T::parse(handle));
                }
                Nlmsg::UnrecognizedVariant(nl_type) => println!("Unrecognized nl_type {}", nl_type),
            };
        }
        responses
    }
}

/// Builder for netlink top level header and payload.
struct NetlinkHeaderBuilder {
    nl_type: u16,
    flags: Vec<NlmF>,
    header_builder: Nl80211HeaderBuilder,
}

impl NetlinkHeaderBuilder {
    fn new(nl_type: u16, header_builder: Nl80211HeaderBuilder) -> Self {
        Self {
            nl_type,
            flags: Vec::new(),
            header_builder,
        }
    }

    fn add_flag(self, flag: NlmF) -> Self {
        let mut nl_flags = self.flags.clone();
        nl_flags.push(flag);
        Self {
            nl_type: self.nl_type,
            flags: nl_flags,
            header_builder: self.header_builder,
        }
    }
}

impl Into<Nlmsghdr<u16, Neli80211Header>> for NetlinkHeaderBuilder {
    fn into(self) -> Nlmsghdr<u16, Neli80211Header> {
        Nlmsghdr::new(
            None,
            self.nl_type,
            self.flags,
            None,
            None,
            self.header_builder.into(),
        )
    }
}

/// Builder for 802.11 netlink header and payload.
struct Nl80211HeaderBuilder {
    command: Command,
    attributes: Vec<(Attribute, Vec<u8>)>,
}

impl Nl80211HeaderBuilder {
    fn new(command: Command) -> Self {
        Self {
            command,
            attributes: Vec::new(),
        }
    }

    fn add_attribute(self, nla_type: Attribute, payload: Vec<u8>) -> Self {
        let mut attributes = self.attributes.clone();
        attributes.push((nla_type, payload));
        Self {
            command: self.command,
            attributes,
        }
    }
}

impl Into<Neli80211Header> for Nl80211HeaderBuilder {
    fn into(self) -> Neli80211Header {
        let attrs = self
            .attributes
            .into_iter()
            .map(|(nla_type, payload)| Nlattr::new(None, nla_type, payload).unwrap())
            .collect();
        Genlmsghdr::new(self.command, NL80211_VERSION, attrs).unwrap()
    }
}

pub(crate) trait Parser {
    fn parse(handle: AttrHandle<Attribute>) -> Self;
}
