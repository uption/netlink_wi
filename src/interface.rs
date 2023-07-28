use std::convert::TryFrom;
use std::fmt;

use log::debug;
use neli::attr::Attribute as NeliAttribute;
use neli::err::DeError;
use neli::FromBytes;

use super::attributes::InterfaceType as NlInterfaceType;
use super::attributes::{Attribute, TxqStats};
use crate::attributes::Attrs;

#[derive(Debug, Clone, Default)]
/// Interface information returned from netlink.
pub struct WirelessInterface {
    /// Index of wiphy to operate on.
    pub wiphy_index: u32,
    /// Network interface index.
    pub interface_index: u32,
    /// Network interface name.
    pub name: String,
    /// Interface MAC address (BSSID).
    pub mac: MacAddress,
    /// Used to indicate consistent snapshots for dumps. This number increases
    /// whenever the object list being dumped changes.
    pub generation: u32,
    /// Network SSID.
    pub ssid: Option<String>,
    /// Channel frequency in MHz.
    pub frequency: Option<u32>,
    /// Offset of the frequncy in KHz.
    pub frequency_offset: Option<u32>,
    /// Center frequency of the first part of the channel, used for anything but 20 MHz bandwidth.
    pub center_frequency1: Option<u32>,
    /// Center frequency of the second part of the channel, used only for 80+80 MHz bandwidth.
    pub center_frequency2: Option<u32>,
    /// Wireless channel width.
    pub channel_width: Option<ChannelWidth>,
    /// Transmit power level (s16) in dBm.
    pub tx_power: Option<u32>,
    /// Wireless device identifier, used for pseudo-devices that don't have a netdev.
    pub wdev: Option<u64>,
    /// Use 4-address frames on a virtual interface.
    pub use_4address_frames: Option<bool>,
    /// Type of virtual interface.
    pub interface_type: Option<InterfaceType>,
    // TXQ statistics.
    pub txq_statistics: Option<TransmitQueueStats>,
}

impl TryFrom<Attrs<'_, Attribute>> for WirelessInterface {
    type Error = DeError;

    fn try_from(handle: Attrs<'_, Attribute>) -> Result<Self, Self::Error> {
        let mut interface = Self::default();
        let mut interface_type_payload: Option<NlInterfaceType> = None;
        let mut txq_stats_attr: Option<Attrs<'_, TxqStats>> = None;
        for attr in handle.iter() {
            match attr.nla_type.nla_type {
                Attribute::Wiphy => interface.wiphy_index = attr.get_payload_as()?,
                Attribute::Ifindex => {
                    interface.interface_index = attr.get_payload_as()?;
                }
                Attribute::Ifname => interface.name = attr.get_payload_as_with_len()?,
                Attribute::Mac => {
                    interface.mac = attr.get_payload_as()?;
                }
                Attribute::Generation => interface.generation = attr.get_payload_as()?,
                Attribute::Ssid => {
                    interface.ssid = Some(String::from_utf8_lossy(attr.payload().as_ref()).into());
                }
                Attribute::WiphyFreq => {
                    interface.frequency = Some(attr.get_payload_as()?);
                }
                Attribute::WiphyChannelType => (), // WiphyChannelType is deprecated.
                Attribute::WiphyFreqOffset => {
                    interface.frequency_offset = Some(attr.get_payload_as()?);
                }
                Attribute::CenterFreq1 => {
                    interface.center_frequency1 = Some(attr.get_payload_as()?);
                }
                Attribute::CenterFreq2 => {
                    interface.center_frequency2 = Some(attr.get_payload_as()?);
                }
                Attribute::ChannelWidth => {
                    let attr_channel_width: u32 = attr.get_payload_as()?;
                    interface.channel_width = Some(attr_channel_width.into());
                }
                Attribute::WiphyTxPowerLevel => {
                    interface.tx_power = Some(attr.get_payload_as()?);
                }
                Attribute::Wdev => {
                    interface.wdev = Some(attr.get_payload_as()?);
                }
                Attribute::Use4addrFrames => {
                    let use_4address_frames: u8 = attr.get_payload_as()?;
                    interface.use_4address_frames = Some(use_4address_frames != 0);
                }
                Attribute::Iftype => {
                    interface_type_payload =
                        Some(handle.get_attr_payload_as::<NlInterfaceType>(Attribute::Iftype)?);
                }
                Attribute::TxqStats => {
                    txq_stats_attr = Some(attr.get_attr_handle()?);
                }
                unhandled => {
                    debug!("Unhandled wireless interface attribute 'Attribute::{unhandled:?}'");
                }
            }
        }
        if let Some(payload) = interface_type_payload {
            match payload {
                NlInterfaceType::Unspecified => {
                    interface.interface_type = Some(InterfaceType::Unspecified);
                }
                NlInterfaceType::Adhoc => {
                    interface.interface_type = Some(InterfaceType::Adhoc);
                }
                NlInterfaceType::Station => {
                    interface.interface_type = Some(InterfaceType::Station);
                }
                NlInterfaceType::Ap => {
                    interface.interface_type = Some(InterfaceType::AccessPoint);
                }
                NlInterfaceType::ApVlan => {
                    interface.interface_type = Some(InterfaceType::ApVlan);
                }
                NlInterfaceType::Wds => {
                    interface.interface_type = Some(InterfaceType::Wds);
                }
                NlInterfaceType::Monitor => {
                    interface.interface_type = Some(InterfaceType::Monitor);
                }
                NlInterfaceType::MeshPoint => {
                    interface.interface_type = Some(InterfaceType::MeshPoint);
                }
                NlInterfaceType::P2pClient => {
                    interface.interface_type = Some(InterfaceType::P2pClient);
                }
                NlInterfaceType::P2pGo => {
                    interface.interface_type = Some(InterfaceType::P2pGroupOwner);
                }
                NlInterfaceType::P2pDevice => {
                    interface.interface_type = Some(InterfaceType::P2pDevice);
                }
                NlInterfaceType::Ocb => {
                    interface.interface_type = Some(InterfaceType::Ocb);
                }
                NlInterfaceType::Nan => {
                    interface.interface_type = Some(InterfaceType::NotNetdev);
                }
                _ => {
                    interface.interface_type = Some(InterfaceType::Unknown);
                }
            }
        }
        if let Some(sub_handle) = txq_stats_attr {
            let mut txq_statistics = TransmitQueueStats::default();
            for sub_attr in sub_handle.iter() {
                match sub_attr.nla_type.nla_type {
                    TxqStats::BacklogBytes => {
                        txq_statistics.backlog_bytes = Some(sub_attr.get_payload_as()?);
                    }
                    TxqStats::BacklogPackets => {
                        txq_statistics.backlog_packets = Some(sub_attr.get_payload_as()?);
                    }
                    TxqStats::Flows => {
                        txq_statistics.flows = Some(sub_attr.get_payload_as()?);
                    }
                    TxqStats::Drops => {
                        txq_statistics.drops = Some(sub_attr.get_payload_as()?);
                    }
                    TxqStats::EcnMarks => {
                        txq_statistics.ecn_marks = Some(sub_attr.get_payload_as()?);
                    }
                    TxqStats::Overlimit => {
                        txq_statistics.overlimit = Some(sub_attr.get_payload_as()?);
                    }
                    TxqStats::Overmemory => {
                        txq_statistics.overmemory = Some(sub_attr.get_payload_as()?);
                    }
                    TxqStats::Collisions => {
                        txq_statistics.collisions = Some(sub_attr.get_payload_as()?);
                    }
                    TxqStats::TxBytes => {
                        txq_statistics.tx_bytes = Some(sub_attr.get_payload_as()?);
                    }
                    TxqStats::TxPackets => {
                        txq_statistics.tx_packets = Some(sub_attr.get_payload_as()?);
                    }
                    TxqStats::MaxFlows => {
                        txq_statistics.max_flows = Some(sub_attr.get_payload_as()?);
                    }
                    unhandled => {
                        debug!("Unhandled txq statistics attribute 'TxqStats::{unhandled:?}'")
                    }
                }
            }
            interface.txq_statistics = Some(txq_statistics);
        }
        Ok(interface)
    }
}

#[derive(Debug, Clone, Default)]
/// Transmit queue statistics.
pub struct TransmitQueueStats {
    /// Number of bytes currently backlogged.
    pub backlog_bytes: Option<u32>,
    /// Number of packets currently backlogged.
    pub backlog_packets: Option<u32>,
    /// Total number of new flows seen.
    pub flows: Option<u32>,
    /// Total number of packet drops.
    pub drops: Option<u32>,
    /// Total number of packet ECN marks.
    pub ecn_marks: Option<u32>,
    /// Number of drops due to queue space overflow.
    pub overlimit: Option<u32>,
    /// Number of drops due to memory limit overflow (only for per-phy stats).
    pub overmemory: Option<u32>,
    /// Number of hash collisions.
    pub collisions: Option<u32>,
    /// Total number of bytes dequeued from TXQ.
    pub tx_bytes: Option<u32>,
    /// Total number of packets dequeued from TXQ.
    pub tx_packets: Option<u32>,
    /// Number of flow buckets for PHY.
    pub max_flows: Option<u32>,
}

#[derive(Debug, Copy, Clone, Default)]
/// MAC-address.
pub struct MacAddress {
    address_bytes: [u8; 6],
}

impl MacAddress {
    pub fn as_bytes(&self) -> [u8; 6] {
        self.address_bytes
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = self
            .address_bytes
            .iter()
            .map(|x| format!("{:02X}", x))
            .collect::<Vec<String>>()
            .join(":");
        write!(f, "{hex}")
    }
}

impl<'a> FromBytes<'a> for MacAddress {
    fn from_bytes(buffer: &mut std::io::Cursor<&'a [u8]>) -> Result<Self, DeError> {
        let address_bytes = buffer
            .clone()
            .into_inner()
            .try_into()
            .map_err(|_| DeError::new("Failed to deserialize MAC-address"))?;
        Ok(MacAddress { address_bytes })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Virtual interface type.
pub enum InterfaceType {
    /// Unspecified type, driver decides.
    Unspecified,
    /// Independent BSS member.
    Adhoc,
    /// Managed BSS member.
    Station,
    /// Access point.
    AccessPoint,
    /// VLAN interface for access points.
    ApVlan,
    /// Wireless distribution interface.
    Wds,
    /// Monitor interface receiving all frames.
    Monitor,
    /// Mesh point.
    MeshPoint,
    /// P2P client.
    P2pClient,
    /// P2P group owner.
    P2pGroupOwner,
    /// P2P device.
    P2pDevice,
    /// Outside Context of a BSS.
    Ocb,
    /// NAN device interface type (not a netdev).
    NotNetdev,
    /// Kernel returned an unknown interface type.
    Unknown,
}

impl fmt::Display for InterfaceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let interface_type = match self {
            InterfaceType::Unspecified => "Unspecified",
            InterfaceType::Adhoc => "Adhoc",
            InterfaceType::Station => "Station",
            InterfaceType::AccessPoint => "Access point",
            InterfaceType::ApVlan => "Access point VLAN interface",
            InterfaceType::Wds => "Wireless distribution interface",
            InterfaceType::Monitor => "Monitor interface",
            InterfaceType::MeshPoint => "Mesh point",
            InterfaceType::P2pClient => "P2P client",
            InterfaceType::P2pGroupOwner => "P2P group owner",
            InterfaceType::P2pDevice => "P2P device",
            InterfaceType::Ocb => "Outside Context of a BSS",
            InterfaceType::NotNetdev => "Not a netdev",
            InterfaceType::Unknown => "Unknown interface type",
        };
        write!(f, "{interface_type}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Wireless channel width.
pub enum ChannelWidth {
    Width20NoHT,
    Width20,
    Width40,
    Width80,
    Width80P80,
    Width160,
    Width5,
    Width10,
    Width1,
    Width2,
    Width4,
    Width8,
    Width16,
    Unknown,
}

impl From<u32> for ChannelWidth {
    fn from(attr_channel_width: u32) -> Self {
        match attr_channel_width {
            0 => ChannelWidth::Width20NoHT,
            1 => ChannelWidth::Width20,
            2 => ChannelWidth::Width40,
            3 => ChannelWidth::Width80,
            4 => ChannelWidth::Width80P80,
            5 => ChannelWidth::Width160,
            6 => ChannelWidth::Width5,
            7 => ChannelWidth::Width10,
            8 => ChannelWidth::Width1,
            9 => ChannelWidth::Width2,
            10 => ChannelWidth::Width4,
            11 => ChannelWidth::Width8,
            12 => ChannelWidth::Width16,
            _ => ChannelWidth::Unknown,
        }
    }
}

impl From<ChannelWidth> for u32 {
    fn from(attr_channel_width: ChannelWidth) -> Self {
        match attr_channel_width {
            ChannelWidth::Width20NoHT => 20,
            ChannelWidth::Width20 => 20,
            ChannelWidth::Width40 => 40,
            ChannelWidth::Width80 => 80,
            ChannelWidth::Width80P80 => 80,
            ChannelWidth::Width160 => 160,
            ChannelWidth::Width5 => 5,
            ChannelWidth::Width10 => 10,
            ChannelWidth::Width1 => 1,
            ChannelWidth::Width2 => 2,
            ChannelWidth::Width4 => 4,
            ChannelWidth::Width8 => 8,
            ChannelWidth::Width16 => 16,
            ChannelWidth::Unknown => 0,
        }
    }
}

impl fmt::Display for ChannelWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let channel_width = match self {
            ChannelWidth::Width20NoHT => "20 MHz non-HT",
            ChannelWidth::Width20 => "20 MHz HT",
            ChannelWidth::Width40 => "40 MHz",
            ChannelWidth::Width80 => "80 MHz",
            ChannelWidth::Width80P80 => "80+80 MHz",
            ChannelWidth::Width160 => "160 MHz",
            ChannelWidth::Width5 => "5 MHz OFDM",
            ChannelWidth::Width10 => "10 MHz OFDM",
            ChannelWidth::Width1 => "1 MHz OFDM",
            ChannelWidth::Width2 => "2 MHz OFDM",
            ChannelWidth::Width4 => "4 MHz OFDM",
            ChannelWidth::Width8 => "8 MHz OFDM",
            ChannelWidth::Width16 => "16 MHz OFDM",
            ChannelWidth::Unknown => "Unknown channel width",
        };
        write!(f, "{channel_width}")
    }
}
