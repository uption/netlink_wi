use neli::consts::nl::NlmF;
use neli::genl::{AttrTypeBuilder, Genlmsghdr, GenlmsghdrBuilder, NlattrBuilder, NoUserHeader};
use neli::nl::NlPayload;
use neli::types::GenlBuffer;

use crate::attributes::Attribute;
use crate::commands::Command;
use crate::interface::{ChannelWidth, InterfaceType};
use crate::MonitorFlags;

use super::attributes::ChannelWidth as NlChannelWidth;
use super::attributes::InterfaceType as NlInterfaceType;

const NL80211_VERSION: u8 = 1;
pub(crate) type Neli80211Header = Genlmsghdr<Command, Attribute>;
pub(crate) struct Nl80211Request {
    pub nl_flags: NlmF,
    pub nl_payload: NlPayload<u16, Neli80211Header>,
}

impl Nl80211Request {
    pub fn list_interfaces() -> Self {
        Self {
            nl_flags: NlmF::REQUEST | NlmF::DUMP,
            nl_payload: NlPayload::Payload(
                GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                    .cmd(Command::GetInterface)
                    .version(NL80211_VERSION)
                    .build()
                    .unwrap(),
            ),
        }
    }

    pub fn get_interface(if_index: u32) -> Self {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Ifindex)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(if_index)
                    .build()
                    .unwrap(),
            );
            attrs
        };
        Self {
            nl_flags: NlmF::REQUEST | NlmF::DUMP,
            nl_payload: NlPayload::Payload(
                GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                    .cmd(Command::GetInterface)
                    .version(NL80211_VERSION)
                    .attrs(attrs)
                    .build()
                    .unwrap(),
            ),
        }
    }

    pub fn set_interface(if_index: u32, if_type: InterfaceType) -> Self {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Ifindex)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(if_index)
                    .build()
                    .unwrap(),
            );
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Iftype)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(Into::<NlInterfaceType>::into(if_type))
                    .build()
                    .unwrap(),
            );
            attrs
        };
        Self {
            nl_flags: NlmF::REQUEST | NlmF::ACK,
            nl_payload: NlPayload::Payload(
                GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                    .cmd(Command::SetInterface)
                    .version(NL80211_VERSION)
                    .attrs(attrs)
                    .build()
                    .unwrap(),
            ),
        }
    }

    pub fn set_monitor_flags(if_index: u32, flags: Vec<MonitorFlags>) -> Self {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Ifindex)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(if_index)
                    .build()
                    .unwrap(),
            );
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Iftype)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(Into::<NlInterfaceType>::into(NlInterfaceType::Monitor))
                    .build()
                    .unwrap(),
            );
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::MntrFlags)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(flags)
                    .build()
                    .unwrap(),
            );
            attrs
        };
        Self {
            nl_flags: NlmF::REQUEST | NlmF::ACK,
            nl_payload: NlPayload::Payload(
                GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                    .cmd(Command::SetInterface)
                    .version(NL80211_VERSION)
                    .attrs(attrs)
                    .build()
                    .unwrap(),
            ),
        }
    }

    pub fn set_channel(if_index: u32, freq: u32, width: ChannelWidth) -> Self {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Ifindex)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(if_index)
                    .build()
                    .unwrap(),
            );
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::WiphyFreq)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(freq)
                    .build()
                    .unwrap(),
            );
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::ChannelWidth)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(Into::<NlChannelWidth>::into(width))
                    .build()
                    .unwrap(),
            );
            attrs
        };
        Self {
            nl_flags: NlmF::REQUEST | NlmF::ACK,
            nl_payload: NlPayload::Payload(
                GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                    .cmd(Command::SetChannel)
                    .version(NL80211_VERSION)
                    .attrs(attrs)
                    .build()
                    .unwrap(),
            ),
        }
    }

    pub fn list_stations(if_index: u32) -> Self {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Ifindex)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(if_index)
                    .build()
                    .unwrap(),
            );
            attrs
        };
        Self {
            nl_flags: NlmF::REQUEST | NlmF::DUMP,
            nl_payload: NlPayload::Payload(
                GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                    .cmd(Command::GetStation)
                    .version(NL80211_VERSION)
                    .attrs(attrs)
                    .build()
                    .unwrap(),
            ),
        }
    }

    pub fn list_physical_devices() -> Self {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::SplitWiphyDump)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(())
                    .build()
                    .unwrap(),
            );
            attrs
        };
        Self {
            nl_flags: NlmF::REQUEST | NlmF::DUMP,
            nl_payload: NlPayload::Payload(
                GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                    .cmd(Command::GetWiphy)
                    .version(NL80211_VERSION)
                    .attrs(attrs)
                    .build()
                    .unwrap(),
            ),
        }
    }

    pub fn get_physical_device(wiphy_index: u32) -> Self {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Wiphy)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(wiphy_index)
                    .build()
                    .unwrap(),
            );
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::SplitWiphyDump)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(())
                    .build()
                    .unwrap(),
            );
            attrs
        };
        Self {
            nl_flags: NlmF::REQUEST | NlmF::DUMP,
            nl_payload: NlPayload::Payload(
                GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                    .cmd(Command::GetWiphy)
                    .version(NL80211_VERSION)
                    .attrs(attrs)
                    .build()
                    .unwrap(),
            ),
        }
    }

    pub fn get_regulatory_domain() -> Self {
        Self {
            nl_flags: NlmF::REQUEST | NlmF::DUMP,
            nl_payload: NlPayload::Payload(
                GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                    .cmd(Command::GetReg)
                    .version(NL80211_VERSION)
                    .build()
                    .unwrap(),
            ),
        }
    }

    pub fn trigger_scan(if_index: u32) -> Self {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Ifindex)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(if_index)
                    .build()
                    .unwrap(),
            );
            attrs
        };
        Self {
            nl_flags: NlmF::REQUEST | NlmF::ACK,
            nl_payload: NlPayload::Payload(
                GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                    .cmd(Command::TriggerScan)
                    .version(NL80211_VERSION)
                    .attrs(attrs)
                    .build()
                    .unwrap(),
            ),
        }
    }

    pub fn abort_scan(if_index: u32) -> Self {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Ifindex)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(if_index)
                    .build()
                    .unwrap(),
            );
            attrs
        };
        Self {
            nl_flags: NlmF::REQUEST | NlmF::ACK,
            nl_payload: NlPayload::Payload(
                GenlmsghdrBuilder::<Command, Attribute, NoUserHeader>::default()
                    .cmd(Command::AbortScan)
                    .version(NL80211_VERSION)
                    .attrs(attrs)
                    .build()
                    .unwrap(),
            ),
        }
    }
}
