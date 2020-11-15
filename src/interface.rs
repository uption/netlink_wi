use std::convert::TryInto;
use std::fmt;

use neli::nlattr::AttrHandle;
use neli::nlattr::Nlattr;

use super::attributes::Attribute;
use super::error::AttrParseError;
use super::netlink::Parser;
use super::netlink::PayloadParser;

#[derive(Debug, Clone, Default)]
/// Interface information returned from netlink.
pub struct WirelessInterface {
    // Network interface index.
    pub index: u32,
    // Network interface name.
    pub name: String,
    // Network SSID.
    pub ssid: Option<String>,
    // Interface MAC address (BSSID).
    pub mac: Option<MacAddress>,
    // Channel frequency in MHz.
    pub frequency: Option<u32>,
    // Center frequency of the first part of the channel, used for anything but 20 MHz bandwidth.
    pub center_frequency1: Option<u32>,
    // Center frequency of the second part of the channel, used only for 80+80 MHz bandwidth.
    pub center_frequency2: Option<u32>,
    // Wireless channel width.
    pub channel_width: Option<ChannelWidth>,
    // Transmit power level (s16) in dBm.
    pub tx_power: Option<u32>,
}

impl Parser for WirelessInterface {
    fn parse(handle: AttrHandle<Attribute>) -> Result<Self, AttrParseError> {
        let mut interface = WirelessInterface::default();
        for attr in handle.iter() {
            match attr.nla_type {
                Attribute::Ifindex => {
                    interface.index = u32::parse(&attr)?;
                }
                Attribute::Ifname => {
                    interface.name = String::from_utf8_lossy(&attr.payload)
                        .trim_matches('\0')
                        .to_string();
                }
                Attribute::Ssid => {
                    interface.ssid = Some(String::from_utf8_lossy(&attr.payload).to_string());
                }
                Attribute::Mac => interface.mac = Some(MacAddress::parse(&attr)?),
                Attribute::WiphyFreq => {
                    interface.frequency = Some(u32::parse(&attr)?);
                }
                Attribute::CenterFreq1 => {
                    interface.center_frequency1 = Some(u32::parse(&attr)?);
                }
                Attribute::CenterFreq2 => {
                    interface.center_frequency2 = Some(u32::parse(&attr)?);
                }
                Attribute::ChannelWidth => {
                    let attr_channel_width = u32::parse(&attr)?;
                    interface.channel_width = Some(attr_channel_width.into());
                }
                Attribute::WiphyTxPowerLevel => {
                    interface.tx_power = Some(u32::parse(&attr)?);
                }
                _ => (),
            }
        }
        Ok(interface)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MacAddress {
    address_bytes: [u8; 6],
}

impl MacAddress {
    fn as_bytes(&self) -> [u8; 6] {
        self.address_bytes
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let foo: Vec<String> = self
            .address_bytes
            .iter()
            .map(|x| format!("{:X?}", x))
            .collect();
        let foo = foo.join(":");
        write!(f, "{}", foo)
    }
}

impl PayloadParser for MacAddress {
    fn parse(attr: &Nlattr<Attribute, Vec<u8>>) -> Result<Self, AttrParseError> {
        let payload: &[u8] = &attr.payload;
        let payload = payload
            .try_into()
            .map_err(|e| AttrParseError::new(e, attr.nla_type.clone()))?;
        Ok(MacAddress {
            address_bytes: payload,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ChannelWidth {
    Width20NoHT,
    Width20,
    Width40,
    Width80,
    Width80P80,
    Width160,
    Width5,
    Width10,
    Width1,
    Width2,
    Width4,
    Width8,
    Width16,
    Unknown,
}

impl From<u32> for ChannelWidth {
    fn from(attr_channel_width: u32) -> Self {
        match attr_channel_width {
            0 => ChannelWidth::Width20NoHT,
            1 => ChannelWidth::Width20,
            2 => ChannelWidth::Width40,
            3 => ChannelWidth::Width80,
            4 => ChannelWidth::Width80P80,
            5 => ChannelWidth::Width160,
            6 => ChannelWidth::Width5,
            7 => ChannelWidth::Width10,
            8 => ChannelWidth::Width1,
            9 => ChannelWidth::Width2,
            10 => ChannelWidth::Width4,
            11 => ChannelWidth::Width8,
            12 => ChannelWidth::Width16,
            _ => ChannelWidth::Unknown,
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
            ChannelWidth::Unknown => "Unknown channel width",
        };
        write!(f, "{}", channel_width)
    }
}
