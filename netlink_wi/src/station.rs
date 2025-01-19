use std::fmt;
use std::time::Duration;

use log::debug;
use neli::attr::Attribute as NeliAttribute;
use neli::err::DeError;

use super::attributes::{
    Attribute, BssParam, HeGuardInterval, HeRuAlloc, RateInfo as NlRateInfo, StationInfo, TidStats,
};
use super::interface::{ChannelWidth, MacAddress, TransmitQueueStats};
use crate::attributes::Attrs;

#[derive(Debug, Clone, Default)]
/// Station information returned from netlink.
pub struct WirelessStation {
    /// Network interface index.
    pub interface_index: u32,
    /// Station MAC address (BSSID).
    pub mac: MacAddress,
    /// Used to indicate consistent snapshots for dumps. This number increases
    /// whenever the object list being dumped changes.
    pub generation: u32,
    /// Signal strength of last received PPDU in dBm.
    pub signal: Option<i8>,
    /// Signal strength average in dBm.
    pub average_signal: Option<i8>,
    /// Signal strength average for beacons only in dBm.
    pub beacon_average_signal: Option<i8>,
    /// Per-chain signal strength of last PPDU in dBm.
    pub chain_signal: Vec<u8>,
    /// Time since the station was last connected.
    pub connected_time: Option<Duration>,
    /// Time since last activity.
    pub inactive_time: Option<Duration>,
    /// Timestamp of station's association after boot (CLOCK_BOOTTIME).
    pub associated_at_boot_time: Option<Duration>,
    /// Total received bytes (MPDU length) from this station.
    pub rx_bytes: Option<u32>,
    /// Total transmitted bytes (MPDU length) to this station.
    pub tx_bytes: Option<u32>,
    /// Total received bytes (MPDU length) from this station.
    pub rx_bytes64: Option<u64>,
    /// Total transmitted bytes (MPDU length) to this station.
    pub tx_bytes64: Option<u64>,
    /// Aggregate PPDU duration for all frames received from the station.
    pub rx_duration: Option<Duration>,
    /// Aggregate PPDU duration for all frames sent to the station.
    pub tx_duration: Option<Duration>,
    /// Total received packet (MSDUs and MMPDUs) from this station.
    pub rx_packets: Option<u32>,
    /// Total transmitted packet (MSDUs and MMPDUs) to this station.
    pub tx_packets: Option<u32>,
    /// Total retries (MPDUs) to this station.
    pub tx_retries: Option<u32>,
    /// Total failed packets (MPDUs) to this station.
    pub tx_failed: Option<u32>,
    /// Count of times beacon loss was detected.
    pub beacon_loss: Option<u32>,
    /// Total received packets dropped for unspecified reasons.
    pub rx_drop_misc: Option<u64>,
    /// Number of beacons received from this station.
    pub beacon_rx: Option<u64>,
    /// Per TID (traffic identifier) statistics.
    pub tid_stats: Option<[TrafficIdStats; 17]>,
    /// Indicates if BSS CTS protection enabled.
    pub bss_cts_protection: Option<bool>,
    /// Indicates if BSS short preamble is enabled.
    pub bss_short_preamble: Option<bool>,
    /// Indicates if short slot time is enabled.
    pub bss_short_slot_time: Option<bool>,
    // BSS DTIM period for beaconing.
    pub bss_dtim_period: Option<u8>,
    // BSS beacon interval.
    pub bss_beacon_interval: Option<u16>,
    // Receive bitrate information.
    pub rx_bitrate: Option<RateInfo>,
    // Transmit bitrate information.
    pub tx_bitrate: Option<RateInfo>,
}

impl TryFrom<&Attrs<'_, Attribute>> for WirelessStation {
    type Error = DeError;

    fn try_from(handle: &Attrs<'_, Attribute>) -> Result<Self, Self::Error> {
        let mut station = WirelessStation::default();
        let mut station_info_attr: Option<Attrs<'_, StationInfo>> = None;
        let mut tid_stats_attr: Option<Attrs<'_, u16>> = None;
        let mut bss_param_attr: Option<Attrs<'_, BssParam>> = None;
        for attr in handle.iter() {
            match attr.nla_type().nla_type() {
                Attribute::Ifindex => station.interface_index = attr.get_payload_as()?,
                Attribute::Mac => station.mac = attr.get_payload_as()?,
                Attribute::Generation => station.generation = attr.get_payload_as()?,
                Attribute::StaInfo => {
                    station_info_attr = Some(attr.get_attr_handle()?);
                }
                unhandled => debug!("Unhandled station attribute 'Attribute::{unhandled:?}'"),
            }
        }

        if let Some(sub_handle) = station_info_attr {
            for sub_attr in sub_handle.iter() {
                match sub_attr.nla_type().nla_type() {
                    StationInfo::Signal => {
                        station.signal = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::SignalAvg => {
                        station.average_signal = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::BeaconSignalAvg => {
                        station.beacon_average_signal = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::ChainSignal => {
                        station.chain_signal = sub_attr.payload().as_ref().to_vec();
                    }
                    StationInfo::ConnectedTime => {
                        let secs: u32 = sub_attr.get_payload_as()?;
                        station.connected_time = Some(Duration::from_secs(secs as u64));
                    }
                    StationInfo::InactiveTime => {
                        let millis: u32 = sub_attr.get_payload_as()?;
                        station.inactive_time = Some(Duration::from_millis(millis as u64));
                    }
                    StationInfo::AssocAtBootTime => {
                        let millis: u64 = sub_attr.get_payload_as()?;
                        station.associated_at_boot_time = Some(Duration::from_nanos(millis));
                    }
                    StationInfo::RxBytes => {
                        station.rx_bytes = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::TxBytes => {
                        station.tx_bytes = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::RxBytes64 => {
                        station.rx_bytes64 = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::TxBytes64 => {
                        station.tx_bytes64 = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::RxDuration => {
                        let millis: u64 = sub_attr.get_payload_as()?;
                        station.rx_duration = Some(Duration::from_millis(millis));
                    }
                    StationInfo::TxDuration => {
                        let millis: u64 = sub_attr.get_payload_as()?;
                        station.tx_duration = Some(Duration::from_millis(millis));
                    }
                    StationInfo::RxPackets => {
                        station.rx_packets = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::TxPackets => {
                        station.tx_packets = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::TxRetries => {
                        station.tx_retries = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::TxFailed => {
                        station.tx_failed = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::BeaconLoss => {
                        station.beacon_loss = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::RxDropMisc => {
                        station.rx_drop_misc = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::BeaconRx => {
                        station.beacon_rx = Some(sub_attr.get_payload_as()?);
                    }
                    StationInfo::StaFlags => (), // TODO: Get station flags
                    StationInfo::RxBitrate => {
                        let sub_handle = sub_attr.get_attr_handle()?;
                        station.rx_bitrate = Some(sub_handle.try_into()?);
                    }
                    StationInfo::TxBitrate => {
                        let sub_handle = sub_attr.get_attr_handle()?;
                        station.tx_bitrate = Some(sub_handle.try_into()?);
                    }
                    StationInfo::TidStats => {
                        tid_stats_attr = Some(sub_attr.get_attr_handle()?);
                    }
                    StationInfo::BssParam => {
                        bss_param_attr = Some(sub_attr.get_attr_handle()?);
                    }
                    unhandled => {
                        debug!("Unhandled station info attribute 'StationInfo::{unhandled:?}'",)
                    }
                }
            }

            if let Some(sub_handle) = tid_stats_attr {
                let mut all_tid_stats: [TrafficIdStats; 17] = Default::default();
                for sub_attr in sub_handle.iter() {
                    let nested_handle = sub_attr.get_attr_handle()?;
                    for tid_attr in nested_handle.iter() {
                        let mut tid_stats = TrafficIdStats::new(*sub_attr.nla_type().nla_type());
                        match tid_attr.nla_type().nla_type() {
                            TidStats::RxMsdu => {
                                tid_stats.rx_msdu = Some(tid_attr.get_payload_as()?);
                            }
                            TidStats::TxMsdu => {
                                tid_stats.tx_msdu = Some(tid_attr.get_payload_as()?);
                            }
                            TidStats::TxMsduRetries => {
                                tid_stats.tx_msdu_retries = Some(tid_attr.get_payload_as()?);
                            }
                            TidStats::TxMsduFailed => {
                                tid_stats.tx_msdu_failed = Some(tid_attr.get_payload_as()?);
                            }
                            TidStats::Pad => (), // Attribute used for padding for 64-bit alignment.
                            TidStats::TxqStats => (), // TODO: Get txq stats.
                            unhandled => {
                                debug!("Unhandled tid stats attribute 'TidStats::{unhandled:?}'")
                            }
                        }
                        all_tid_stats[*sub_attr.nla_type().nla_type() as usize - 1] = tid_stats;
                    }
                }
                station.tid_stats = Some(all_tid_stats);
            }

            if let Some(sub_handle) = bss_param_attr {
                station.bss_cts_protection = Some(false);
                station.bss_short_preamble = Some(false);
                station.bss_short_slot_time = Some(false);
                for sub_attr in sub_handle.iter() {
                    match sub_attr.nla_type().nla_type() {
                        BssParam::CtsProt => {
                            station.bss_cts_protection = Some(true);
                        }
                        BssParam::ShortPreamble => {
                            station.bss_short_preamble = Some(true);
                        }
                        BssParam::ShortSlotTime => {
                            station.bss_short_slot_time = Some(true);
                        }
                        BssParam::DtimPeriod => {
                            station.bss_dtim_period = Some(sub_attr.get_payload_as()?);
                        }
                        BssParam::BeaconInterval => {
                            station.bss_beacon_interval = Some(sub_attr.get_payload_as()?);
                        }
                        unhandled => {
                            debug!("Unhandled BSS param attribute 'BssParam::{unhandled:?}'")
                        }
                    }
                }
            }
        }
        Ok(station)
    }
}

#[derive(Debug, Clone, Default)]
/// Traffic identifier statistics.
pub struct TrafficIdStats {
    /// TID number 1-16 and 17 for non-QoS traffic.
    pub tid_number: u16,
    /// Number of MSDUs received.
    pub rx_msdu: Option<u64>,
    /// Number of MSDUs transmitted or attempted to transmit.
    pub tx_msdu: Option<u64>,
    /// Number of retries for transmitted MSDUs (not counting the first attempt).
    pub tx_msdu_retries: Option<u64>,
    /// Number of failed transmitted MSDUs.
    pub tx_msdu_failed: Option<u64>,
    /// TXQ statistics.
    pub txq_stats: Option<TransmitQueueStats>,
}

impl TrafficIdStats {
    pub fn new(tid_number: u16) -> Self {
        Self {
            tid_number,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
/// Station bitrate information.
pub struct RateInfo {
    /// Bitrate in 100kbit/s.
    pub bitrate: u32,
    /// MCS index.
    pub mcs: u8,
    /// Connection or frame type.
    pub connection_type: ConnectionType,
    /// Guard interval.
    pub guard_interval: GuardIntervals,
    /// Channel width.
    pub channel_width: ChannelWidth,
    /// Number of spatial streams.
    stream_count: u8,
    /// HE DCM value (0/1).
    dcm_value: Option<u8>,
    /// HE RU allocation, if not present then non-OFDMA is used.
    ru_allocation: Option<HeRuAllocation>,
}

impl TryFrom<Attrs<'_, NlRateInfo>> for RateInfo {
    type Error = DeError;

    fn try_from(handle: Attrs<'_, NlRateInfo>) -> Result<Self, Self::Error> {
        let mut bitrate_info = Self {
            bitrate: 0,
            mcs: 0,
            connection_type: ConnectionType::Unknown,
            // Default guard interval for HT & VHT when short GI is not set.
            guard_interval: GuardIntervals::Usec0_8,
            // Default channel width with HT if no other flags set.
            channel_width: ChannelWidth::Width20,
            stream_count: 0,
            dcm_value: None,
            ru_allocation: None,
        };
        for attr in handle.iter() {
            match attr.nla_type().nla_type() {
                NlRateInfo::Bitrate => {
                    if bitrate_info.bitrate == 0 {
                        let bitrate: u16 = attr.get_payload_as()?;
                        bitrate_info.bitrate = bitrate as u32;
                    }
                }
                NlRateInfo::Mcs => {
                    bitrate_info.connection_type = ConnectionType::Ht;
                    bitrate_info.mcs = attr.get_payload_as()?;
                    if bitrate_info.mcs < 8 {
                        bitrate_info.stream_count = 1;
                    } else if bitrate_info.mcs < 16 {
                        bitrate_info.stream_count = 2;
                    } else if bitrate_info.mcs < 24 {
                        bitrate_info.stream_count = 3;
                    } else {
                        bitrate_info.stream_count = 4;
                    }
                }
                NlRateInfo::MhzWidth40 => {
                    bitrate_info.channel_width = ChannelWidth::Width40;
                }
                NlRateInfo::ShortGuardInterval => {
                    bitrate_info.guard_interval = GuardIntervals::Usec0_4;
                }
                NlRateInfo::Bitrate32 => {
                    bitrate_info.bitrate = attr.get_payload_as()?;
                }
                NlRateInfo::VhtMcs => {
                    bitrate_info.mcs = attr.get_payload_as()?;
                    bitrate_info.connection_type = ConnectionType::Vht;
                }
                NlRateInfo::VhtNss => {
                    bitrate_info.stream_count = attr.get_payload_as()?;
                }
                NlRateInfo::MhzWidth80 => {
                    bitrate_info.channel_width = ChannelWidth::Width80;
                }
                NlRateInfo::MhzWidth80p80 => {
                    bitrate_info.channel_width = ChannelWidth::Width80P80;
                }
                NlRateInfo::MhzWidth160 => {
                    bitrate_info.channel_width = ChannelWidth::Width160;
                }
                NlRateInfo::MhzWidth10 => {
                    bitrate_info.channel_width = ChannelWidth::Width10;
                }
                NlRateInfo::MhzWidth5 => {
                    bitrate_info.channel_width = ChannelWidth::Width5;
                }
                NlRateInfo::HeMcs => {
                    bitrate_info.mcs = attr.get_payload_as()?;
                    bitrate_info.connection_type = ConnectionType::He;
                }
                NlRateInfo::HeNss => {
                    bitrate_info.stream_count = attr.get_payload_as()?;
                }
                NlRateInfo::HeGuardInterval => {
                    let payload = handle
                        .get_attr_payload_as::<HeGuardInterval>(NlRateInfo::HeGuardInterval)?;
                    bitrate_info.guard_interval = match payload {
                        HeGuardInterval::Usec0_8 => GuardIntervals::Usec0_8,
                        HeGuardInterval::Usec1_6 => GuardIntervals::Usec1_6,
                        HeGuardInterval::Usec3_2 => GuardIntervals::Usec3_2,
                        unknown => {
                            debug!("Unknown HE guard interval attribute {unknown:?}");
                            GuardIntervals::Unknown
                        }
                    }
                }
                NlRateInfo::HeDcm => {
                    bitrate_info.dcm_value = Some(attr.get_payload_as()?);
                }
                NlRateInfo::HeRuAlloc => {
                    let payload = handle.get_attr_payload_as::<HeRuAlloc>(NlRateInfo::HeRuAlloc)?;
                    let ru_allocation = match payload {
                        HeRuAlloc::Alloc26 => HeRuAllocation::Alloc26,
                        HeRuAlloc::Alloc52 => HeRuAllocation::Alloc52,
                        HeRuAlloc::Alloc106 => HeRuAllocation::Alloc106,
                        HeRuAlloc::Alloc242 => HeRuAllocation::Alloc242,
                        HeRuAlloc::Alloc484 => HeRuAllocation::Alloc484,
                        HeRuAlloc::Alloc996 => HeRuAllocation::Alloc996,
                        HeRuAlloc::Alloc2x996 => HeRuAllocation::Alloc2x996,
                        unknown => {
                            debug!("Unknown HE RU allocation attribute {unknown:?}");
                            HeRuAllocation::Unknown
                        }
                    };
                    bitrate_info.ru_allocation = Some(ru_allocation);
                }
                unhandled => {
                    debug!("Unhandled rate info attribute 'NlRateInfo::{unhandled:?}'");
                }
            }
        }
        Ok(bitrate_info)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Connection or frame type.
pub enum ConnectionType {
    /// High Throughput (802.11n).
    Ht,
    /// Very High Throughput (802.11ac).
    Vht,
    /// High Efficiency (802.11ax).
    He,
    /// Unknown connection type.
    Unknown,
}

impl fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_string = match self {
            Self::Ht => "HT",
            Self::Vht => "VHT",
            Self::He => "HE",
            Self::Unknown => "Unknown",
        };
        write!(f, "{}", as_string)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Wifi connection guard intervals.
pub enum GuardIntervals {
    /// 0.4 microseconds.
    Usec0_4,
    /// 0.8 microseconds.
    Usec0_8,
    /// 1.6 microseconds.
    Usec1_6,
    /// 3.2 microseconds.
    Usec3_2,
    /// Unknown guard interval.
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// HE RU allocation values.
pub enum HeRuAllocation {
    /// 26-tone RU allocation.
    Alloc26,
    /// 52-tone RU allocation.
    Alloc52,
    /// 106-tone RU allocation.
    Alloc106,
    /// 242-tone RU allocation.
    Alloc242,
    /// 484-tone RU allocation.
    Alloc484,
    /// 996-tone RU allocation.
    Alloc996,
    /// 2x996-tone RU allocation.
    Alloc2x996,
    /// Unknown RU allocation.
    Unknown,
}
