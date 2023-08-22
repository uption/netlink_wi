use log::debug;
use neli::attr::Attribute as NeliAttribute;
use neli::err::DeError;

use super::attributes::Attribute;
use crate::attributes::{Attrs, RegRuleAttr, RegRuleFlags};

#[derive(Debug, Clone, Default)]
/// Regulatory domain information returned from netlink.
///
/// If wiphy_index is specified and the device has a private regulatory domain,
/// it will be returned. Otherwise, the global regdomain will be returned.
pub struct RegulatoryDomain {
    /// ISO/IEC 3166-1 alpha2 country code.
    pub country_code: String,
    /// Region for regulatory rules which this country abides to when initiating
    /// radiation on DFS channels.
    pub dfs_region: DfsRegion,
    /// Device index.
    pub wiphy_index: Option<u32>,
    /// Indicates if device is self-managing its regulatory information.
    pub self_managed: bool,
}

impl TryFrom<&Attrs<'_, Attribute>> for RegulatoryDomain {
    type Error = DeError;

    fn try_from(handle: &Attrs<'_, Attribute>) -> Result<Self, Self::Error> {
        let mut reg_domain = Self::default();
        let mut reg_rule_attr = Vec::new();
        for attr in handle.iter() {
            match attr.nla_type().nla_type() {
                Attribute::RegAlpha2 => reg_domain.country_code = attr.get_payload_as_with_len()?,
                Attribute::DfsRegion => {
                    reg_domain.dfs_region = match attr.get_payload_as::<u8>()? {
                        1 => DfsRegion::Fcc,
                        2 => DfsRegion::Etsi,
                        3 => DfsRegion::JP,
                        _ => DfsRegion::Unset,
                    }
                }
                Attribute::Wiphy => reg_domain.wiphy_index = Some(attr.get_payload_as()?),
                Attribute::RegRules => {
                    let sub_handle: Attrs<'_, u16> = attr.get_attr_handle()?;
                    for sub_attr in sub_handle.iter() {
                        let freq_handle: Attrs<'_, RegRuleAttr> = sub_attr.get_attr_handle()?;
                        let freq: RegulatoryRule = freq_handle.try_into()?;
                        reg_rule_attr.push(freq);
                    }
                }
                Attribute::WiphySelfManagedReg => reg_domain.self_managed = true,
                unhandled => {
                    debug!("Unhandled regulatory domain attribute 'Attribute::{unhandled:?}'")
                }
            }
        }
        Ok(reg_domain)
    }
}

#[derive(Debug, Clone, Default)]
/// Regulatory rule information returned from netlink.
pub struct RegulatoryRule {
    /// Starting frequencry for the regulatory rule in KHz.
    /// This is not a center of frequency but an actual regulatory band edge.
    pub freq_range_start: u32,
    /// Ending frequency for the regulatory rule in KHz.
    /// This is not a center a frequency but an actual regulatory band edge.
    pub freq_range_end: u32,
    /// Maximum allowed bandwidth for this frequency range, in KHz.
    pub max_bandwidth: u32,
    /// The maximum allowed antenna gain for a given frequency range.
    /// The value is in mBi (100 * dBi).
    pub max_antenna_gain: u32,
    /// The maximum allowed EIRP for a given frequency range.
    /// The value is in mBm (100 * dBm).
    pub max_eirp: u32,
    /// DFS CAC time in milliseconds.
    /// If not present or 0 default CAC time will be used.
    pub dfs_cac_time: u32,
    /// OFDM modulation not allowed.
    pub no_ofdm: bool,
    /// CCK modulation not allowed.
    pub no_cck: bool,
    /// Indoor operation not allowed.
    pub no_indoor: bool,
    /// Outdoor operation not allowed.
    pub no_outdoor: bool,
    /// DFS support is required to be used.
    pub dfs: bool,
    /// This is only for Point To Point links.
    pub ptp_only: bool,
    /// This is only for Point To Multi Point links.
    pub pmtp_only: bool,
    /// No mechanisms that initiate radiation are allowed, this includes probe
    /// requests or modes of operation that require beaconing.
    pub no_ir: bool,
    /// IR operation is allowed on this channel if it's connected concurrently
    /// to a BSS on the same channel on the 2 GHz band or to a channel in the
    /// same UNII band (on the 5 GHz band), and radar detection is not required.
    pub ir_concurrent: bool,
    /// Maximum available bandwidth should be calculated based on contiguous
    /// rules and wider channels will be allowed to cross multiple
    /// contiguous/overlapping frequency ranges.
    pub auto_bandwidth: bool,
    /// HT40- operation not allowed.
    pub no_ht40_minus: bool,
    /// HT40+ operation not allowed.
    pub no_ht40_plus: bool,
    /// 80MHz operation not allowed
    pub no_80mhz: bool,
    /// 160MHz operation not allowed.
    pub no_160mhz: bool,
    /// 320MHz operation not allowed.
    pub no_320mhz: bool,
    /// HE operation not allowed.
    pub no_he: bool,
    /// EHT operation not allowed.
    pub no_eht: bool,
}

impl TryFrom<Attrs<'_, RegRuleAttr>> for RegulatoryRule {
    type Error = DeError;

    fn try_from(handle: Attrs<'_, RegRuleAttr>) -> Result<Self, Self::Error> {
        let mut reg_rule = Self::default();
        for attr in handle.iter() {
            match attr.nla_type().nla_type() {
                RegRuleAttr::RegRuleFlags => {
                    if let Some(flags) = RegRuleFlags::from_bits(attr.get_payload_as()?) {
                        reg_rule.no_ofdm = flags.contains(RegRuleFlags::NO_OFDM);
                        reg_rule.no_cck = flags.contains(RegRuleFlags::NO_CCK);
                        reg_rule.no_indoor = flags.contains(RegRuleFlags::NO_INDOOR);
                        reg_rule.no_outdoor = flags.contains(RegRuleFlags::NO_OUTDOOR);
                        reg_rule.dfs = flags.contains(RegRuleFlags::DFS);
                        reg_rule.ptp_only = flags.contains(RegRuleFlags::PTP_ONLY);
                        reg_rule.pmtp_only = flags.contains(RegRuleFlags::PTMP_ONLY);
                        reg_rule.no_ir = flags.contains(RegRuleFlags::NO_IR);
                        reg_rule.ir_concurrent = flags.contains(RegRuleFlags::IR_CONCURRENT);
                        reg_rule.auto_bandwidth = flags.contains(RegRuleFlags::AUTO_BW);
                        reg_rule.no_ht40_minus = flags.contains(RegRuleFlags::NO_HT40MINUS);
                        reg_rule.no_ht40_plus = flags.contains(RegRuleFlags::NO_HT40PLUS);
                        reg_rule.no_80mhz = flags.contains(RegRuleFlags::NO_80MHZ);
                        reg_rule.no_160mhz = flags.contains(RegRuleFlags::NO_160MHZ);
                        reg_rule.no_320mhz = flags.contains(RegRuleFlags::NO_320MHZ);
                        reg_rule.no_he = flags.contains(RegRuleFlags::NO_HE);
                        reg_rule.no_eht = flags.contains(RegRuleFlags::NO_EHT);
                    }
                }
                RegRuleAttr::FreqRangeStart => reg_rule.freq_range_start = attr.get_payload_as()?,
                RegRuleAttr::FreqRangeEnd => reg_rule.freq_range_end = attr.get_payload_as()?,
                RegRuleAttr::FreqRangeMaxBw => reg_rule.max_bandwidth = attr.get_payload_as()?,
                RegRuleAttr::PowerRuleMaxAntGain => {
                    reg_rule.max_antenna_gain = attr.get_payload_as()?
                }
                RegRuleAttr::PowerRuleMaxEirp => reg_rule.max_eirp = attr.get_payload_as()?,
                RegRuleAttr::DfsCacTime => reg_rule.dfs_cac_time = attr.get_payload_as()?,
                unhandled => {
                    debug!("Unhandled regulatory rule attribute 'Attribute::{unhandled:?}'")
                }
            }
        }
        Ok(reg_rule)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Region for regulatory rules which this country abides to when initiating
/// radiation on DFS channels.
pub enum DfsRegion {
    /// Country has no DFS master region specified.
    #[default]
    Unset,
    /// Country follows DFS master rules from FCC.
    Fcc,
    /// Country follows DFS master rules from ETSI.
    Etsi,
    /// Country follows DFS master rules from JP/MKK/Telec.
    JP,
}
