use neli::nlattr::AttrHandle;

use crate::attributes::WiphyBands;

use super::attributes::Attribute;
use super::error::AttrParseError;
use super::netlink::AttributeParser;
use super::netlink::PayloadParser;

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
}

impl AttributeParser<Attribute> for PhysicalDevice {
    fn parse(handle: AttrHandle<Attribute>) -> Result<Self, AttrParseError> {
        let mut device = PhysicalDevice::default();
        let mut wiphy_bands_attr: Option<AttrHandle<'_, WiphyBands>> = None;

        for attr in handle.iter() {
            match &attr.nla_type {
                Attribute::Wiphy => {
                    device.wiphy_index = u32::parse(attr)?;
                }
                Attribute::WiphyName => {
                    device.name = String::from_utf8_lossy(&attr.payload)
                        .trim_matches('\0')
                        .to_string();
                }
                Attribute::Generation => device.generation = u32::parse(attr)?,
                Attribute::WiphyBands => {
                    wiphy_bands_attr = Some(
                        attr.get_nested_attributes::<WiphyBands>()
                            .map_err(|err| AttrParseError::new(err, Attribute::WiphyBands))?,
                    );
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
                match &sub_attr.nla_type {
                    WiphyBands::Invalid => (),
                    unhandled => println!("Unhandled wiphy band 'Attribute::{:?}'", &unhandled),
                }
            }
        }
        Ok(device)
    }
}
