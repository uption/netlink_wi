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

mod asynchronous;
pub mod interface;
mod netlink;
pub mod reg_domain;
pub mod station;
mod synchronous;
pub mod wiphy;

pub use crate::attributes::MonitorFlags;
pub use asynchronous::AsyncNlSocket;
pub use synchronous::NlSocket;
