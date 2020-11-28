use neli::nlattr::AttrHandle;
use std::time::Duration;

use super::attributes::{Attribute, BssParam, StationInfo, TidStats};
use super::error::AttrParseError;
use super::interface::{MacAddress, TransmitQueueStats};
use super::netlink::AttributeParser;
use super::netlink::PayloadParser;

#[derive(Debug, Clone, Default)]
/// Station information returned from netlink.
pub struct Station {
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
    // TxBitrate
    // RxBitrate
    // BssParam
    // StaFlags
}

impl AttributeParser for Station {
    fn parse(handle: AttrHandle<Attribute>) -> Result<Self, AttrParseError> {
        let mut station = Station::default();
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
                    match &sub_attr.nla_type {
                        i => {
                            let nested_handle = sub_attr
                                .get_nested_attributes::<TidStats>()
                                .map_err(|err| AttrParseError::new(err, StationInfo::TidStats))?;
                            for tid_attr in nested_handle.iter() {
                                let mut tid_stats = TrafficIdStats::new(*i as u8);
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
                                    TidStats::TxqStats => (), // TODO
                                    unhandled => println!(
                                        "Unhandled tid stats attribute 'TidStats::{:?}'",
                                        &unhandled
                                    ),
                                }
                                all_tid_stats[*i as usize - 1] = tid_stats;
                            }
                        }
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
                                format!("Unhandled BSS param attribute"),
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
    pub tid_number: u8,
    /// Number of MSDUs received (u64).
    pub rx_msdu: Option<u64>,
    /// Number of MSDUs transmitted or attempted to transmit (u64).
    pub tx_msdu: Option<u64>,
    /// Number of retries for transmitted MSDUs (not counting the first attempt; u64).
    pub tx_msdu_retries: Option<u64>,
    /// Number of failed transmitted MSDUs (u64).
    pub tx_msdu_failed: Option<u64>,
    /// TXQ stats (nested attribute).
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
