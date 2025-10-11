//! **netlink_wi** is a Rust library for interacting with wireless devices on Linux systems through the **nl80211** netlink interface.
//!
//! It provides programmatic access to wireless configuration and status information, allowing developers to query and control Wi-Fi interfaces directly from Rust code.
//!
//! With **netlink_wi**, you can:
//!
//! - Enumerate and inspect wireless network interfaces
//! - Retrieve hardware capabilities and detailed device specifications
//! - Access information about connected stations and peers
//! - Query wireless regulatory domain settings
//! - Configure wireless interface parameters
//!
//! ## Usage
//!
//! Synchronous example:
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
//! Asynchronous example (requires `async` feature):
//!
//! ```rust,no_run
//! use netlink_wi::AsyncNlSocket;
//!
//! async fn list_interfaces() {
//!    let socket = AsyncNlSocket::connect().await.unwrap();
//!    let interfaces = socket.list_interfaces().await.unwrap();
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

#[cfg(feature = "async")]
mod asynchronous;
mod error;
pub mod interface;
mod netlink;
pub mod reg_domain;
pub mod station;
#[cfg(feature = "sync")]
mod synchronous;
pub mod wiphy;

pub use crate::attributes::MonitorFlags;
#[cfg(feature = "async")]
pub use asynchronous::AsyncNlSocket;
pub use error::NlError;
pub use error::Result as NlResult;
pub use netlink::ChannelConfig;
#[cfg(feature = "sync")]
pub use synchronous::NlSocket;
