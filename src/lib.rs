//! A library to retrieve information about wireless hardware in Linux operating
//! system using netlink protocol.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use netlink_wi::NlSocket;
//!
//! fn list_interfaces() {
//!    let mut socket = NlSocket::connect().unwrap();
//!    let interfaces = socket.list_interfaces().unwrap();
//!    for interface in interfaces {
//!        println!("{:#?}", interface);
//!    }
//! }
//! ```
//!
//! See more examples in [Github](https://github.com/uption/netlink_wi/tree/master/examples).
//!
pub(crate) mod attributes;
pub(crate) mod commands;

mod interface;
mod netlink;
mod reg_domain;
mod station;
mod wiphy;

pub use interface::{
    ChannelWidth, InterfaceType, MacAddress, TransmitQueueStats, WirelessInterface,
};
pub use netlink::NlSocket;
pub use station::{
    ConnectionType, GuardIntervals, HeRuAllocation, RateInfo, TrafficIdStats, WirelessStation,
};
