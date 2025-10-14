//! Wireless network interface types and information.
//!
//! This module defines types for working with wireless network interfaces, including
//! interface configuration, operating modes, channel settings, and MAC addresses.

use std::convert::{TryFrom, TryInto};
use std::fmt;

use log::debug;
use neli::attr::Attribute as NeliAttribute;
use neli::err::DeError;
use neli::FromBytes;

use super::attributes::ChannelWidth as NlChannelWidth;
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
    pub channel_width: ChannelWidth,
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
    /// Supported BSS membership selectors exposed by the supplicant.
    pub supported_selectors: Option<Vec<u8>>,
}

impl TryFrom<&Attrs<'_, Attribute>> for WirelessInterface {
    type Error = DeError;

    fn try_from(handle: &Attrs<'_, Attribute>) -> Result<Self, Self::Error> {
        let mut interface = Self::default();
        let mut interface_type_payload: Option<NlInterfaceType> = None;
        let mut txq_stats_attr: Option<Attrs<'_, TxqStats>> = None;
        for attr in handle.iter() {
            match attr.nla_type().nla_type() {
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
                    let attr_channel_width: NlChannelWidth = attr.get_payload_as()?;
                    interface.channel_width = attr_channel_width.into();
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
                Attribute::SupportedSelectors => {
                    interface.supported_selectors = Some(attr.payload().as_ref().to_vec());
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
            interface.txq_statistics = Some(sub_handle.try_into()?);
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

impl TryFrom<Attrs<'_, TxqStats>> for TransmitQueueStats {
    type Error = DeError;

    fn try_from(handle: Attrs<'_, TxqStats>) -> Result<Self, Self::Error> {
        let mut stats = TransmitQueueStats::default();
        for attr in handle.iter() {
            match attr.nla_type().nla_type() {
                TxqStats::BacklogBytes => {
                    stats.backlog_bytes = Some(attr.get_payload_as()?);
                }
                TxqStats::BacklogPackets => {
                    stats.backlog_packets = Some(attr.get_payload_as()?);
                }
                TxqStats::Flows => {
                    stats.flows = Some(attr.get_payload_as()?);
                }
                TxqStats::Drops => {
                    stats.drops = Some(attr.get_payload_as()?);
                }
                TxqStats::EcnMarks => {
                    stats.ecn_marks = Some(attr.get_payload_as()?);
                }
                TxqStats::Overlimit => {
                    stats.overlimit = Some(attr.get_payload_as()?);
                }
                TxqStats::Overmemory => {
                    stats.overmemory = Some(attr.get_payload_as()?);
                }
                TxqStats::Collisions => {
                    stats.collisions = Some(attr.get_payload_as()?);
                }
                TxqStats::TxBytes => {
                    stats.tx_bytes = Some(attr.get_payload_as()?);
                }
                TxqStats::TxPackets => {
                    stats.tx_packets = Some(attr.get_payload_as()?);
                }
                TxqStats::MaxFlows => {
                    stats.max_flows = Some(attr.get_payload_as()?);
                }
                unhandled => {
                    debug!("Unhandled txq statistics attribute 'TxqStats::{unhandled:?}'");
                }
            }
        }
        Ok(stats)
    }
}

#[derive(Debug, Copy, Clone, Default)]
/// MAC-address.
pub struct MacAddress {
    address_bytes: [u8; 6],
}

impl MacAddress {
    /// Returns the MAC address as a byte array.
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

impl FromBytes for MacAddress {
    fn from_bytes(buffer: &mut std::io::Cursor<impl AsRef<[u8]>>) -> Result<Self, DeError> {
        let address_bytes = buffer
            .get_ref()
            .as_ref()
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

impl From<InterfaceType> for NlInterfaceType {
    fn from(value: InterfaceType) -> Self {
        match value {
            InterfaceType::Unspecified => NlInterfaceType::Unspecified,
            InterfaceType::Adhoc => NlInterfaceType::Adhoc,
            InterfaceType::Station => NlInterfaceType::Station,
            InterfaceType::AccessPoint => NlInterfaceType::Ap,
            InterfaceType::ApVlan => NlInterfaceType::ApVlan,
            InterfaceType::Wds => NlInterfaceType::Wds,
            InterfaceType::Monitor => NlInterfaceType::Monitor,
            InterfaceType::MeshPoint => NlInterfaceType::MeshPoint,
            InterfaceType::P2pClient => NlInterfaceType::P2pClient,
            InterfaceType::P2pGroupOwner => NlInterfaceType::P2pGo,
            InterfaceType::P2pDevice => NlInterfaceType::P2pDevice,
            InterfaceType::Ocb => NlInterfaceType::Ocb,
            InterfaceType::NotNetdev => NlInterfaceType::Nan,
            InterfaceType::Unknown => NlInterfaceType::Unspecified,
        }
    }
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
/// Wireless channel width.
pub enum ChannelWidth {
    /// 20 MHz channel without HT (High Throughput) features.
    Width20NoHT,
    /// 20 MHz channel with HT features.
    Width20,
    /// 40 MHz channel.
    Width40,
    /// 80 MHz channel.
    Width80,
    /// 80+80 MHz non-contiguous channel.
    Width80P80,
    /// 160 MHz channel.
    Width160,
    /// 5 MHz OFDM channel.
    Width5,
    /// 10 MHz OFDM channel.
    Width10,
    /// 1 MHz OFDM channel.
    Width1,
    /// 2 MHz OFDM channel.
    Width2,
    /// 4 MHz OFDM channel.
    Width4,
    /// 8 MHz OFDM channel.
    Width8,
    /// 16 MHz OFDM channel.
    Width16,
    /// 320 MHz channel.
    Width320,
    /// Unknown or unrecognized channel width.
    #[default]
    Unknown,
}

impl From<ChannelWidth> for NlChannelWidth {
    fn from(attr_channel_width: ChannelWidth) -> Self {
        match attr_channel_width {
            ChannelWidth::Width20NoHT => NlChannelWidth::Width20NoHT,
            ChannelWidth::Width20 => NlChannelWidth::Width20,
            ChannelWidth::Width40 => NlChannelWidth::Width40,
            ChannelWidth::Width80 => NlChannelWidth::Width80,
            ChannelWidth::Width80P80 => NlChannelWidth::Width80P80,
            ChannelWidth::Width160 => NlChannelWidth::Width160,
            ChannelWidth::Width5 => NlChannelWidth::Width5,
            ChannelWidth::Width10 => NlChannelWidth::Width10,
            ChannelWidth::Width1 => NlChannelWidth::Width1,
            ChannelWidth::Width2 => NlChannelWidth::Width2,
            ChannelWidth::Width4 => NlChannelWidth::Width4,
            ChannelWidth::Width8 => NlChannelWidth::Width8,
            ChannelWidth::Width16 => NlChannelWidth::Width16,
            ChannelWidth::Width320 => NlChannelWidth::Width320,
            ChannelWidth::Unknown => NlChannelWidth::Width20NoHT,
        }
    }
}

impl From<NlChannelWidth> for ChannelWidth {
    fn from(attr_channel_width: NlChannelWidth) -> Self {
        match attr_channel_width {
            NlChannelWidth::Width20NoHT => ChannelWidth::Width20NoHT,
            NlChannelWidth::Width20 => ChannelWidth::Width20,
            NlChannelWidth::Width40 => ChannelWidth::Width40,
            NlChannelWidth::Width80 => ChannelWidth::Width80,
            NlChannelWidth::Width80P80 => ChannelWidth::Width80P80,
            NlChannelWidth::Width160 => ChannelWidth::Width160,
            NlChannelWidth::Width5 => ChannelWidth::Width5,
            NlChannelWidth::Width10 => ChannelWidth::Width10,
            NlChannelWidth::Width1 => ChannelWidth::Width1,
            NlChannelWidth::Width2 => ChannelWidth::Width2,
            NlChannelWidth::Width4 => ChannelWidth::Width4,
            NlChannelWidth::Width8 => ChannelWidth::Width8,
            NlChannelWidth::Width16 => ChannelWidth::Width16,
            NlChannelWidth::Width320 => ChannelWidth::Width320,
            NlChannelWidth::UnrecognizedConst(_) => ChannelWidth::Unknown,
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
            ChannelWidth::Width320 => "320 MHz",
            ChannelWidth::Unknown => "Unknown channel width",
        };
        write!(f, "{channel_width}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mac_address_from_bytes_and_display() {
        let mac_bytes = [0x00, 0x11, 0x22, 0xAA, 0xBB, 0xCC];
        let mut cursor = std::io::Cursor::new(mac_bytes);

        let mac = MacAddress::from_bytes(&mut cursor).expect("mac bytes deserialize");

        assert_eq!(mac.as_bytes(), mac_bytes);
        assert_eq!(mac.to_string(), "00:11:22:AA:BB:CC");
    }
}
