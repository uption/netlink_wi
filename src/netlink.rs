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

    /// Set the channel for the given interface.
    pub fn set_channel(config: ChannelConfig) -> Self {
        let attrs = {
            let mut attrs = GenlBuffer::new();
            let attr_type = AttrTypeBuilder::default()
                .nla_type(Attribute::Ifindex)
                .build()
                .unwrap();
            attrs.push(
                NlattrBuilder::default()
                    .nla_type(attr_type)
                    .nla_payload(config.if_index)
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
                    .nla_payload(config.freq)
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
                    .nla_payload(Into::<NlChannelWidth>::into(config.width))
                    .build()
                    .unwrap(),
            );

            if let Some(center_freq1) = config.center_freq1 {
                let attr_type = AttrTypeBuilder::default()
                    .nla_type(Attribute::CenterFreq1)
                    .build()
                    .unwrap();
                attrs.push(
                    NlattrBuilder::default()
                        .nla_type(attr_type)
                        .nla_payload(center_freq1)
                        .build()
                        .unwrap(),
                );
            }
            if let Some(center_freq2) = config.center_freq2 {
                let attr_type = AttrTypeBuilder::default()
                    .nla_type(Attribute::CenterFreq2)
                    .build()
                    .unwrap();
                attrs.push(
                    NlattrBuilder::default()
                        .nla_type(attr_type)
                        .nla_payload(center_freq2)
                        .build()
                        .unwrap(),
                );
            }

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

/// Configuration for setting a channel.
///
/// Center frequency 1 is required for the following channel widths:
/// - 40 MHz
/// - 80 MHz
/// - 80+80 MHz
/// - 160 MHz
/// - 320 MHz
///
/// Center frequency 2 is required for the following channel widths:
/// - 80+80 MHz
///
#[derive(Debug, Clone)]
pub struct ChannelConfig {
    if_index: u32,
    freq: u32,
    center_freq1: Option<u32>,
    center_freq2: Option<u32>,
    width: ChannelWidth,
}

impl ChannelConfig {
    pub fn new(if_index: u32, freq: u32, width: ChannelWidth) -> Self {
        Self {
            if_index,
            freq,
            center_freq1: None,
            center_freq2: None,
            width,
        }
    }

    pub fn with_center_freq1(mut self, center_freq1: u32) -> Self {
        self.center_freq1 = Some(center_freq1);
        self
    }

    pub fn with_center_freq2(mut self, center_freq2: u32) -> Self {
        self.center_freq2 = Some(center_freq2);
        self
    }
}
