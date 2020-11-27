use neli::nlattr::AttrHandle;
use std::time::Duration;

use super::attributes::{Attribute, StationInfo};
use super::error::AttrParseError;
use super::interface::MacAddress;
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
    // TidStats
    // TxBitrate
    // TxBitrate
    // BssParam
    // StaFlags
}

impl AttributeParser for Station {
    fn parse(handle: AttrHandle<Attribute>) -> Result<Self, AttrParseError> {
        let mut station = Station::default();
        for attr in handle.iter() {
            let mut station_info: Option<AttrHandle<'_, StationInfo>> = None;
            match &attr.nla_type {
                Attribute::Ifindex => {
                    station.interface_index = u32::parse(&attr)?;
                }
                Attribute::Mac => station.mac = MacAddress::parse(&attr)?,
                Attribute::Generation => station.generation = u32::parse(&attr)?,
                Attribute::StaInfo => {
                    station_info = Some(
                        attr.get_nested_attributes::<StationInfo>()
                            .map_err(|err| AttrParseError::new(err, Attribute::StaInfo))?,
                    );
                }
                unhandled => println!("Unhandled station attribute 'Attribute::{:?}'", &unhandled),
            }

            if let Some(sub_handle) = station_info {
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
                            station.rx_duration =
                                Some(Duration::from_millis(u64::parse(&sub_attr)?));
                        }
                        StationInfo::TxDuration => {
                            station.tx_duration =
                                Some(Duration::from_millis(u64::parse(&sub_attr)?));
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
                        unhandled => println!(
                            "Unhandled station info attribute 'StationInfo::{:?}'",
                            &unhandled
                        ),
                    }
                }
            }
        }

        Ok(station)
    }
}
