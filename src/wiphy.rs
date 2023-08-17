use log::debug;
use neli::attr::Attribute as NeliAttribute;
use neli::err::DeError;

use super::attributes::Attribute;
use crate::{
    attributes::{Attrs, Band, BandAttr, FrequencyAttr},
    interface::MacAddress,
};

#[derive(Debug, Clone, Default)]
/// Physical wireless device information returned from netlink.
pub struct PhysicalDevice {
    /// Device index.
    pub wiphy_index: u32,
    /// Device name.
    pub name: String,
    /// Used to indicate consistent snapshots for dumps. This number increases
    /// whenever the object list being dumped changes.
    pub generation: u32,
    /// 2.4 GHz band.
    pub band_2ghz: Option<WifiBand>,
    /// 5 GHz band.
    pub band_5ghz: Option<WifiBand>,
    /// 6 GHz band.
    pub band_6ghz: Option<WifiBand>,
    /// Indicates if device is self-managing its regulatory information.
    pub self_managed_reg: bool,
    /// Device MAC address (BSSID).
    pub mac: Option<MacAddress>,
}

impl PhysicalDevice {
    pub(crate) fn merge(&mut self, other: &Self) {
        if other.self_managed_reg {
            self.self_managed_reg = true;
        }
        if other.mac.is_some() {
            self.mac = other.mac;
        }
        if let Some(other_band_2ghz) = &other.band_2ghz {
            if let Some(self_band_2ghz) = &mut self.band_2ghz {
                self_band_2ghz
                    .frequencies
                    .extend(other_band_2ghz.frequencies.clone());
            } else {
                self.band_2ghz = other.band_2ghz.clone();
            }
        }
        if let Some(other_band_5ghz) = &other.band_5ghz {
            if let Some(self_band_5ghz) = &mut self.band_5ghz {
                self_band_5ghz
                    .frequencies
                    .extend(other_band_5ghz.frequencies.clone());
            } else {
                self.band_5ghz = other.band_5ghz.clone();
            }
        }
        if let Some(other_band_6ghz) = &other.band_6ghz {
            if let Some(self_band_6ghz) = &mut self.band_6ghz {
                self_band_6ghz
                    .frequencies
                    .extend(other_band_6ghz.frequencies.clone());
            } else {
                self.band_6ghz = other.band_6ghz.clone();
            }
        }
    }
}

impl TryFrom<Attrs<'_, Attribute>> for PhysicalDevice {
    type Error = DeError;

    fn try_from(handle: Attrs<'_, Attribute>) -> Result<Self, Self::Error> {
        let mut device = PhysicalDevice::default();
        let mut wiphy_bands_attr: Option<Attrs<'_, Band>> = None;

        for attr in handle.iter() {
            match attr.nla_type().nla_type() {
                Attribute::Wiphy => device.wiphy_index = attr.get_payload_as()?,
                Attribute::WiphyName => device.name = attr.get_payload_as_with_len()?,
                Attribute::Generation => device.generation = attr.get_payload_as()?,
                Attribute::WiphyBands => {
                    wiphy_bands_attr = Some(attr.get_attr_handle()?);
                }
                Attribute::Mac => {
                    device.mac = Some(attr.get_payload_as()?);
                }
                Attribute::WiphySelfManagedReg => device.self_managed_reg = true,
                Attribute::WiphyRetryShort
                | Attribute::WiphyRetryLong
                | Attribute::WiphyFragThreshold
                | Attribute::WiphyRtsThreshold
                | Attribute::WiphyCoverageClass
                | Attribute::MaxNumScanSsids
                | Attribute::MaxNumSchedScanSsids
                | Attribute::MaxScanIeLen
                | Attribute::MaxSchedScanIeLen
                | Attribute::MaxMatchSets
                | Attribute::MaxScanPlanInterval
                | Attribute::MaxScanPlanIterations
                | Attribute::MaxNumSchedScanPlans
                | Attribute::SchedScanMaxReqs
                | Attribute::SupportIbssRsn
                | Attribute::SupportApUapsd
                | Attribute::TdlsSupport
                | Attribute::TdlsExternalSetup
                | Attribute::CipherSuites
                | Attribute::MaxNumPmkids
                | Attribute::ControlPortEthertype
                | Attribute::WiphyAntennaAvailTx
                | Attribute::WiphyAntennaAvailRx
                | Attribute::WiphyAntennaTx
                | Attribute::WiphyAntennaRx
                | Attribute::SupportedIftypes
                | Attribute::SupportedCommands
                | Attribute::MaxRemainOnChannelDuration
                | Attribute::OffchannelTxOk
                | Attribute::WowlanTriggersSupported
                | Attribute::SoftwareIftypes
                | Attribute::InterfaceCombinations
                | Attribute::FeatureFlags
                | Attribute::HtCapabilityMask
                | Attribute::EmlCapability
                | Attribute::PeerMeasurements
                | Attribute::RxFrameTypes
                | Attribute::TxFrameTypes
                | Attribute::TxqQuantum
                | Attribute::TxqMemoryLimit
                | Attribute::TxqLimit
                | Attribute::TxqStats
                | Attribute::NanDual
                | Attribute::IftypeExtCapa
                | Attribute::ExtFeatures
                | Attribute::ExtCapa
                | Attribute::ExtCapaMask
                | Attribute::MaxCsaCounters
                | Attribute::VhtCapabilityMask
                | Attribute::SarSpec
                | Attribute::MacAddrs => (), // TODO: Implement all wiphy attributes.
                unhandled => debug!("Unhandled station attribute 'Attribute::{unhandled:?}'"),
            }
        }
        if let Some(sub_handle) = wiphy_bands_attr {
            for sub_attr in sub_handle.iter() {
                match sub_attr.nla_type().nla_type() {
                    Band::Band2ghz => {
                        let sub_handle: Attrs<'_, BandAttr> = sub_attr.get_attr_handle()?;
                        device.band_2ghz = Some(sub_handle.try_into()?);
                    }
                    Band::Band5ghz => {
                        let sub_handle: Attrs<'_, BandAttr> = sub_attr.get_attr_handle()?;
                        device.band_5ghz = Some(sub_handle.try_into()?);
                    }
                    Band::Band6ghz => {
                        let sub_handle: Attrs<'_, BandAttr> = sub_attr.get_attr_handle()?;
                        device.band_6ghz = Some(sub_handle.try_into()?);
                    }
                    Band::Band60ghz | Band::BandS1ghz | Band::BandLc => (),
                    unhandled => debug!("Unhandled band 'Band::{unhandled:?}'"),
                }
            }
        }
        Ok(device)
    }
}

#[derive(Debug, Clone, Default)]
/// Wi-Fi band.
pub struct WifiBand {
    /// Supported frequencies in MHz.
    pub frequencies: Vec<Frequency>,
}

impl TryFrom<Attrs<'_, BandAttr>> for WifiBand {
    type Error = DeError;

    fn try_from(handle: Attrs<'_, BandAttr>) -> Result<Self, Self::Error> {
        let mut band = WifiBand::default();
        for attr in handle.iter() {
            match attr.nla_type().nla_type() {
                BandAttr::Frequencies => {
                    let sub_handle: Attrs<'_, u16> = attr.get_attr_handle()?;
                    for sub_attr in sub_handle.iter() {
                        let freq_handle: Attrs<'_, FrequencyAttr> = sub_attr.get_attr_handle()?;
                        let freq: Frequency = freq_handle.try_into()?;
                        band.frequencies.push(freq);
                    }
                }
                BandAttr::Bitrates
                | BandAttr::HtMcsSet
                | BandAttr::HtCapabilities
                | BandAttr::HtAmpduFactor
                | BandAttr::HtAmpduDensity
                | BandAttr::VhtMcsSet
                | BandAttr::VhtCapabilities
                | BandAttr::IftypeData
                | BandAttr::EdmgChannels
                | BandAttr::EdmgBwConfig => (), // TODO: Implement all band attributes.
                unhandled => debug!("Unhandled band attribute 'BandAttr::{unhandled:?}'"),
            }
        }
        Ok(band)
    }
}

#[derive(Debug, Clone, Default)]
/// Frequency information.
pub struct Frequency {
    /// Frequency in MHz.
    pub frequency: u32,
    /// Channel is disabled in current regulatory domain.
    pub disabled: bool,
    /// No mechanisms that initiate radiation are permitted on this channel.
    pub no_ir: bool,
    /// Radar detection is mandatory on this channel in current regulatory domain.
    pub radar_detection: bool,
    /// Maximum transmission power in mBm (100 * dBm).
    pub max_tx_power: u32,
}

impl TryFrom<Attrs<'_, FrequencyAttr>> for Frequency {
    type Error = DeError;

    fn try_from(handle: Attrs<'_, FrequencyAttr>) -> Result<Self, Self::Error> {
        let mut frequency = Frequency::default();
        for attr in handle.iter() {
            match attr.nla_type().nla_type() {
                FrequencyAttr::Frequency => {
                    frequency.frequency = attr.get_payload_as()?;
                }
                FrequencyAttr::Disabled => {
                    frequency.disabled = true;
                }
                FrequencyAttr::NoIr | FrequencyAttr::NoIbss => {
                    frequency.no_ir = true;
                }
                FrequencyAttr::Radar => {
                    frequency.radar_detection = true;
                }
                FrequencyAttr::MaxTxPower => {
                    frequency.max_tx_power = attr.get_payload_as()?;
                }
                FrequencyAttr::DfsState
                | FrequencyAttr::DfdTime
                | FrequencyAttr::NoHt40Minus
                | FrequencyAttr::NoHt40Plus
                | FrequencyAttr::No80Mhz
                | FrequencyAttr::No160Mhz
                | FrequencyAttr::DfsCacTime
                | FrequencyAttr::IndoorOnly
                | FrequencyAttr::IrConcurrent
                | FrequencyAttr::No20Mhz
                | FrequencyAttr::No10Mhz
                | FrequencyAttr::Wmm
                | FrequencyAttr::NoHe
                | FrequencyAttr::Offset
                | FrequencyAttr::Allow1Mhz
                | FrequencyAttr::Allow2Mhz
                | FrequencyAttr::Allow4Mhz
                | FrequencyAttr::Allow8Mhz
                | FrequencyAttr::Allow16Mhz
                | FrequencyAttr::No320Mhz
                | FrequencyAttr::NoEht => (), // TODO: Implement all frequency attributes.
                unhandled => {
                    debug!("Unhandled frequency attribute 'FrequencyAttr::{unhandled:?}'",)
                }
            }
        }
        Ok(frequency)
    }
}
