use std::cell::RefCell;
use std::convert::TryInto;
use std::fmt;

use neli::consts::nl::{NlTypeWrapper, NlmF, NlmFFlags, Nlmsg};
use neli::consts::socket::NlFamily;
use neli::err::NlError;
use neli::genl::Genlmsghdr;
use neli::nl::{NlPayload, Nlmsghdr};
use neli::socket::NlSocketHandle;
use neli::types::{Buffer, GenlBuffer};
use neli::utils::U32Bitmask;
use neli::{attr::AttrHandle, consts::genl::NlAttrType};
use neli::{consts::genl::NlAttrTypeWrapper, genl::Nlattr};

use super::attributes::Attribute;
use super::commands::Command;
use super::error::AttrParseError;
use super::interface::WirelessInterface;

const NL80211_VERSION: u8 = 1;
pub struct NlSocket {
    socket: RefCell<NlSocketHandle>,
    nl_type: u16,
}

impl NlSocket {
    /// Connect netlink socket.
    pub fn connect() -> Result<Self, NlError> {
        let mut socket = NlSocketHandle::connect(NlFamily::Generic, None, U32Bitmask::empty())?;
        let nl_type = socket.resolve_genl_family("nl80211")?;
        Ok(Self {
            socket: RefCell::new(socket),
            nl_type,
        })
    }

    pub fn list_interfaces(&self) -> Result<Vec<Result<WirelessInterface, NlError>>, NlError> {
        let attrs = GenlBuffer::<NlAttrTypeWrapper, Buffer>::new();
        let genlhdr = Genlmsghdr::new(Command::GetInterface, NL80211_VERSION, attrs);
        let nlhdr = Nlmsghdr::new(
            None,
            self.nl_type,
            NlmFFlags::new(&[NlmF::Request, NlmF::Dump]),
            None,
            None,
            NlPayload::Payload(genlhdr),
        );
        self.socket.borrow_mut().send(nlhdr)?;

        let mut interfaces = Vec::new();
        for response_result in self
            .socket
            .borrow_mut()
            .iter::<Genlmsghdr<Command, Attribute>>(false)
        {
            let response = response_result?;
            if let NlTypeWrapper::Nlmsg(Nlmsg::Error) = response.nl_type {
                return Err(NlError::new(
                    "An error occurred while retrieving available families",
                ));
            }
            let handle = response.get_payload()?.get_attr_handle();
            interfaces.push(WirelessInterface::parse(handle));
        }

        Ok(interfaces)
    }

    // pub fn list_stations(
    //     &self,
    //     if_index: u32,
    // ) -> Result<Vec<Result<Station, AttrParseError>>, NlError> {
    //     let mut attrs = GenlBuffer::<NlAttrTypeWrapper, Buffer>::new();
    //     attrs.push(Nlattr::new(
    //         None,
    //         false,
    //         true,
    //         Attribute::Ifindex,
    //         if_index.to_le_bytes().to_vec(),
    //     ));
    //     let nl_payload = Genlmsghdr::new(Command::GetStation, NL80211_VERSION, attrs)
    //         .expect("Failed to create generic netlink header and payload");
    //     let msg = Nlmsghdr::new(
    //         None,
    //         self.nl_type,
    //         NlmFFlags::new(&[NlmF::Request, NlmF::Dump]),
    //         None,
    //         None,
    //         nl_payload,
    //     );
    //     self.socket.borrow_mut().send(msg)?;
    //     self.read::<Station>()
    // }

    // // fn send(&self, payload: NetlinkHeaderBuilder) -> Result<(), NlError> {
    // //     self.socket.borrow_mut().send_nl(payload.into())
    // // }

    // fn read<P: AttributeParser<Attribute>>(
    //     &self,
    // ) -> Result<Vec<Result<P, AttrParseError>>, NlError> {
    //     let mut responses = Vec::new();
    //     for response in self
    //         .socket
    //         .borrow_mut()
    //         .iter::<Genlmsghdr<Command, Attribute>>()
    //     {
    //         let response = response?;
    //         match response.nl_type {
    //             Nlmsg::Noop => (),
    //             Nlmsg::Overrun => (),
    //             Nlmsg::Done => break,
    //             Nlmsg::Error => {
    //                 println!("Kernel returned an error");
    //                 break;
    //             }
    //             Nlmsg::UnrecognizedVariant(nl_type) if nl_type == self.nl_type => {
    //                 let handle = response.nl_payload.get_attr_handle();
    //                 responses.push(P::parse(handle));
    //             }
    //             Nlmsg::UnrecognizedVariant(nl_type) => println!("Unrecognized nl_type {}", nl_type),
    //         };
    //     }
    //     Ok(responses)
    // }
}

pub(crate) trait AttributeParser<T> {
    fn parse(
        handle: AttrHandle<GenlBuffer<Attribute, Buffer>, Nlattr<Attribute, Buffer>>,
    ) -> Result<Self, NlError>
    where
        T: NlAttrType,
        Self: Sized;
}
