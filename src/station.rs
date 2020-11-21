use neli::nlattr::AttrHandle;

use super::attributes::{Attribute, StationInfo};
use super::error::AttrParseError;
use super::netlink::AttributeParser;
use super::netlink::PayloadParser;

#[derive(Debug, Clone, Default)]
/// Station information returned from netlink.
pub struct Station {
    pub average_signal: Option<u8>,
}

impl AttributeParser for Station {
    fn parse(handle: AttrHandle<Attribute>) -> Result<Self, AttrParseError> {
        let mut station = Station::default();
        for attr in handle.iter() {
            match attr.nla_type {
                Attribute::StaInfo => {
                    let sub_handle = attr
                        .get_nested_attributes::<StationInfo>()
                        .map_err(|err| AttrParseError::new(err, Attribute::StaInfo))?;
                    for sub_attr in sub_handle.iter() {
                        match sub_attr.nla_type {
                            StationInfo::SignalAvg => {
                                station.average_signal = Some(u8::parse(&sub_attr)?);
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
        Ok(station)
    }
}
