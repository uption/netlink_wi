use neli::nlattr::AttrHandle;
use std::time::Duration;

use super::attributes::{
    Attribute, BssParam, HeGuardInterval, HeRuAlloc, RateInfo as NlRateInfo, StationInfo, TidStats,
};
use super::error::AttrParseError;
use super::interface::{ChannelWidth, MacAddress, TransmitQueueStats};
use super::netlink::AttributeParser;
use super::netlink::PayloadParser;

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
    pub signal: Option<u8>,
    /// Signal strength average in dBm.
    pub average_signal: Option<u8>,
    /// Signal strength average for beacons only in dBm.
    pub beacon_average_signal: Option<u8>,
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

impl AttributeParser<Attribute> for WirelessStation {
    fn parse(handle: AttrHandle<Attribute>) -> Result<Self, AttrParseError> {
        let mut station = WirelessStation::default();
        let mut station_info_attr: Option<AttrHandle<'_, StationInfo>> = None;
        let mut tid_stats_attr: Option<AttrHandle<'_, u16>> = None;
        let mut bss_param_attr: Option<AttrHandle<'_, BssParam>> = None;
        for attr in handle.iter() {
            match &attr.nla_type {
                Attribute::Ifindex => {
                    station.interface_index = u32::parse(&attr)?;
                }
                Attribute::Mac => station.mac = MacAddress::parse(&attr)?,
                Attribute::Generation => station.generation = u32::parse(&attr)?,
                Attribute::StaInfo => {
                    station_info_attr = Some(
                        attr.get_nested_attributes::<StationInfo>()
                            .map_err(|err| AttrParseError::new(err, Attribute::StaInfo))?,
                    );
                }
                unhandled => println!("Unhandled station attribute 'Attribute::{:?}'", &unhandled),
            }
        }

        if let Some(sub_handle) = station_info_attr {
            for sub_attr in sub_handle.iter() {
                match &sub_attr.nla_type {
                    StationInfo::Signal => {
                        station.signal = Some(u8::parse(&sub_attr)?);
                    }
                    StationInfo::SignalAvg => {
                        station.average_signal = Some(u8::parse(&sub_attr)?);
                    }
                    StationInfo::BeaconSignalAvg => {
                        station.beacon_average_signal = Some(u8::parse(&sub_attr)?);
                    }
                    StationInfo::ChainSignal => {
                        station.chain_signal = sub_attr.payload.to_vec();
                    }
                    StationInfo::ConnectedTime => {
                        station.connected_time =
                            Some(Duration::from_secs(u32::parse(&sub_attr)? as u64));
                    }
                    StationInfo::InactiveTime => {
                        station.inactive_time =
                            Some(Duration::from_millis(u32::parse(&sub_attr)? as u64));
                    }
                    StationInfo::AssocAtBootTime => {
                        station.associated_at_boot_time =
                            Some(Duration::from_nanos(u64::parse(&sub_attr)?));
                    }
                    StationInfo::RxBytes => {
                        station.rx_bytes = Some(u32::parse(&sub_attr)?);
                    }
                    StationInfo::TxBytes => {
                        station.tx_bytes = Some(u32::parse(&sub_attr)?);
                    }
                    StationInfo::RxBytes64 => {
                        station.rx_bytes64 = Some(u64::parse(&sub_attr)?);
                    }
                    StationInfo::TxBytes64 => {
                        station.tx_bytes64 = Some(u64::parse(&sub_attr)?);
                    }
                    StationInfo::RxDuration => {
                        station.rx_duration = Some(Duration::from_millis(u64::parse(&sub_attr)?));
                    }
                    StationInfo::TxDuration => {
                        station.tx_duration = Some(Duration::from_millis(u64::parse(&sub_attr)?));
                    }
                    StationInfo::RxPackets => {
                        station.rx_packets = Some(u32::parse(&sub_attr)?);
                    }
                    StationInfo::TxPackets => {
                        station.tx_packets = Some(u32::parse(&sub_attr)?);
                    }
                    StationInfo::TxRetries => {
                        station.tx_retries = Some(u32::parse(&sub_attr)?);
                    }
                    StationInfo::TxFailed => {
                        station.tx_failed = Some(u32::parse(&sub_attr)?);
                    }
                    StationInfo::BeaconLoss => {
                        station.beacon_loss = Some(u32::parse(&sub_attr)?);
                    }
                    StationInfo::RxDropMisc => {
                        station.rx_drop_misc = Some(u64::parse(&sub_attr)?);
                    }
                    StationInfo::BeaconRx => {
                        station.beacon_rx = Some(u64::parse(&sub_attr)?);
                    }
                    StationInfo::StaFlags => (), // TODO: Get station flags
                    StationInfo::RxBitrate => {
                        let sub_handle = sub_attr
                            .get_nested_attributes::<NlRateInfo>()
                            .map_err(|err| AttrParseError::new(err, StationInfo::TxBitrate))?;
                        station.rx_bitrate = Some(RateInfo::parse(sub_handle)?);
                    }
                    StationInfo::TxBitrate => {
                        let sub_handle = sub_attr
                            .get_nested_attributes::<NlRateInfo>()
                            .map_err(|err| AttrParseError::new(err, StationInfo::TxBitrate))?;
                        station.tx_bitrate = Some(RateInfo::parse(sub_handle)?);
                    }
                    StationInfo::TidStats => {
                        tid_stats_attr = Some(
                            sub_attr
                                .get_nested_attributes::<u16>()
                                .map_err(|err| AttrParseError::new(err, StationInfo::TidStats))?,
                        );
                    }
                    StationInfo::BssParam => {
                        bss_param_attr = Some(
                            sub_attr
                                .get_nested_attributes::<BssParam>()
                                .map_err(|err| AttrParseError::new(err, StationInfo::BssParam))?,
                        );
                    }
                    unhandled => println!(
                        "Unhandled station info attribute 'StationInfo::{:?}'",
                        &unhandled
                    ),
                }
            }

            if let Some(sub_handle) = tid_stats_attr {
                let mut all_tid_stats: [TrafficIdStats; 17] = Default::default();
                for sub_attr in sub_handle.iter() {
                    let nested_handle = sub_attr
                        .get_nested_attributes::<TidStats>()
                        .map_err(|err| AttrParseError::new(err, StationInfo::TidStats))?;
                    for tid_attr in nested_handle.iter() {
                        let mut tid_stats = TrafficIdStats::new(sub_attr.nla_type as u8);
                        match &tid_attr.nla_type {
                            TidStats::RxMsdu => {
                                tid_stats.rx_msdu = Some(u64::parse(&tid_attr)?);
                            }
                            TidStats::TxMsdu => {
                                tid_stats.tx_msdu = Some(u64::parse(&tid_attr)?);
                            }
                            TidStats::TxMsduRetries => {
                                tid_stats.tx_msdu_retries = Some(u64::parse(&tid_attr)?);
                            }
                            TidStats::TxMsduFailed => {
                                tid_stats.tx_msdu_failed = Some(u64::parse(&tid_attr)?);
                            }
                            TidStats::Pad => (), // Attribute used for padding for 64-bit alignment.
                            TidStats::TxqStats => (), // TODO: Get txq stats.
                            unhandled => println!(
                                "Unhandled tid stats attribute 'TidStats::{:?}'",
                                &unhandled
                            ),
                        }
                        all_tid_stats[sub_attr.nla_type as usize - 1] = tid_stats;
                    }
                }
                station.tid_stats = Some(all_tid_stats);
            }

            if let Some(sub_handle) = bss_param_attr {
                station.bss_cts_protection = Some(false);
                station.bss_short_preamble = Some(false);
                station.bss_short_slot_time = Some(false);
                for sub_attr in sub_handle.iter() {
                    match &sub_attr.nla_type {
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
                            station.bss_dtim_period = Some(u8::parse(&sub_attr)?);
                        }
                        BssParam::BeaconInterval => {
                            station.bss_beacon_interval = Some(u16::parse(&sub_attr)?);
                        }
                        unhandled => {
                            return Err(AttrParseError::new(
                                "Unhandled BSS param attribute",
                                unhandled,
                            ));
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
    pub tid_number: u8,
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
    pub fn new(tid_number: u8) -> Self {
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

impl AttributeParser<NlRateInfo> for RateInfo {
    fn parse(handle: AttrHandle<NlRateInfo>) -> Result<Self, AttrParseError> {
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
            match &attr.nla_type {
                NlRateInfo::Bitrate => {
                    if bitrate_info.bitrate == 0 {
                        bitrate_info.bitrate = u16::parse(&attr)? as u32;
                    }
                }
                NlRateInfo::Mcs => {
                    bitrate_info.connection_type = ConnectionType::HT;
                    bitrate_info.mcs = u8::parse(&attr)?;
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
                    bitrate_info.bitrate = u32::parse(&attr)?;
                }
                NlRateInfo::VhtMcs => {
                    bitrate_info.mcs = u8::parse(&attr)?;
                    bitrate_info.connection_type = ConnectionType::VHT;
                }
                NlRateInfo::VhtNss => {
                    bitrate_info.stream_count = u8::parse(&attr)?;
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
                    bitrate_info.mcs = u8::parse(&attr)?;
                    bitrate_info.connection_type = ConnectionType::HE;
                }
                NlRateInfo::HeNss => {
                    bitrate_info.stream_count = u8::parse(&attr)?;
                }
                NlRateInfo::HeGuardInterval => {
                    let payload = handle
                        .get_attr_payload_as::<HeGuardInterval>(NlRateInfo::HeGuardInterval)
                        .map_err(|err| {
                            AttrParseError::new(err.to_string(), NlRateInfo::HeGuardInterval)
                        })?;
                    bitrate_info.guard_interval = match payload {
                        HeGuardInterval::Usec0_8 => GuardIntervals::Usec0_8,
                        HeGuardInterval::Usec1_6 => GuardIntervals::Usec1_6,
                        HeGuardInterval::Usec3_2 => GuardIntervals::Usec3_2,
                        unknown => {
                            println!("Unknown HE guard interval attribute {:?}", unknown);
                            GuardIntervals::Unknown
                        }
                    }
                }
                NlRateInfo::HeDcm => {
                    bitrate_info.dcm_value = Some(u8::parse(&attr)?);
                }
                NlRateInfo::HeRuAlloc => {
                    let payload = handle
                        .get_attr_payload_as::<HeRuAlloc>(NlRateInfo::HeRuAlloc)
                        .map_err(|err| {
                            AttrParseError::new(err.to_string(), NlRateInfo::HeRuAlloc)
                        })?;
                    let ru_allocation = match payload {
                        HeRuAlloc::Alloc26 => HeRuAllocation::Alloc26,
                        HeRuAlloc::Alloc52 => HeRuAllocation::Alloc52,
                        HeRuAlloc::Alloc106 => HeRuAllocation::Alloc106,
                        HeRuAlloc::Alloc242 => HeRuAllocation::Alloc242,
                        HeRuAlloc::Alloc484 => HeRuAllocation::Alloc484,
                        HeRuAlloc::Alloc996 => HeRuAllocation::Alloc996,
                        HeRuAlloc::Alloc2x996 => HeRuAllocation::Alloc2x996,
                        unknown => {
                            println!("Unknown HE RU allocation attribute {:?}", unknown);
                            HeRuAllocation::Unknown
                        }
                    };
                    bitrate_info.ru_allocation = Some(ru_allocation);
                }
                unhandled => {
                    println!("Unhandled rate info attribute {:?}", unhandled);
                }
            }
        }
        Ok(bitrate_info)
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionType {
    /// High Throughput (802.11n).
    HT,
    /// Very High Throughput (802.11ac).
    VHT,
    /// High Efficiency (802.11ax).
    HE,
    /// Unknown connection type.
    Unknown,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
