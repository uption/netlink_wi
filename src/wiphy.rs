use neli::attr::Attribute as NeliAttribute;
use neli::err::DeError;

use super::attributes::Attribute;
use crate::attributes::{Attrs, Band, BandAttr, FrequencyAttr};

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
    pub band_2ghz: Option<WifiBand>,
    pub band_5ghz: Option<WifiBand>,
    pub band_6ghz: Option<WifiBand>,
}

impl TryFrom<Attrs<'_, Attribute>> for PhysicalDevice {
    type Error = DeError;

    fn try_from(handle: Attrs<'_, Attribute>) -> Result<Self, Self::Error> {
        let mut device = PhysicalDevice::default();
        let mut wiphy_bands_attr: Option<Attrs<'_, Band>> = None;

        for attr in handle.iter() {
            match attr.nla_type.nla_type {
                Attribute::Wiphy => device.wiphy_index = attr.get_payload_as()?,
                Attribute::WiphyName => device.name = attr.get_payload_as_with_len()?,
                Attribute::Generation => device.generation = attr.get_payload_as()?,
                Attribute::WiphyBands => {
                    wiphy_bands_attr = Some(attr.get_attr_handle()?);
                }
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
                | Attribute::HtCapabilityMask => (), // TODO: Implement all wiphy attributes.
                unhandled => println!("Unhandled station attribute 'Attribute::{:?}'", &unhandled),
            }
        }
        if let Some(sub_handle) = wiphy_bands_attr {
            for sub_attr in sub_handle.iter() {
                match sub_attr.nla_type.nla_type {
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
                    unhandled => println!("Unhandled band 'Attribute::{:?}'", &unhandled),
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
    pub frequencies: Vec<u32>,
}

impl TryFrom<Attrs<'_, BandAttr>> for WifiBand {
    type Error = DeError;

    fn try_from(handle: Attrs<'_, BandAttr>) -> Result<Self, Self::Error> {
        let mut band = WifiBand::default();
        for attr in handle.iter() {
            if attr.nla_type.nla_type == BandAttr::Frequencies {
                let sub_handle: Attrs<'_, u16> = attr.get_attr_handle()?;
                for sub_attr in sub_handle.iter() {
                    let freq_handle: Attrs<'_, FrequencyAttr> = sub_attr.get_attr_handle()?;
                    for freq_attr in freq_handle.iter() {
                        if freq_attr.nla_type.nla_type == FrequencyAttr::Frequency {
                            let freq: u32 = freq_attr.get_payload_as()?;
                            band.frequencies.push(freq);
                        }
                    }
                }
            }
        }
        Ok(band)
    }
}
