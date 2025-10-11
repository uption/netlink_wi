use std::collections::HashMap;
use std::fmt::Write;
use std::io::Cursor;

use log::debug;
use neli::consts::nl::Nlmsg;
use neli::consts::socket::NlFamily;
use neli::err::RouterError;
use neli::nl::NlPayload;
use neli::router::synchronous::{NlRouter, NlRouterReceiverHandle};
use neli::utils::Groups;
use neli::ToBytes;

use crate::attributes::{Attribute, Attrs, MonitorFlags};
use crate::error::Result;
use crate::interface::InterfaceType;
use crate::netlink::{ChannelConfig, Neli80211Header, Nl80211Request};
use crate::reg_domain::RegulatoryDomain;
use crate::station::WirelessStation;
use crate::wiphy::PhysicalDevice;

use super::interface::WirelessInterface;

/// A synchronous netlink socket for interacting with nl80211.
///
/// This socket provides a synchronous interface to nl80211 (the Linux kernel's 802.11
/// wireless configuration API). It allows you to query and configure wireless
/// interfaces, physical devices, stations, and regulatory domains.
///
/// # Examples
///
/// ```no_run
/// # use netlink_wi::NlSocket;
/// # fn example() -> netlink_wi::NlResult<()> {
/// let socket = NlSocket::connect()?;
/// let interfaces = socket.list_interfaces()?;
/// # Ok(())
/// # }
/// ```
pub struct NlSocket {
    socket: NlRouter,
    nl_type: u16,
}

impl NlSocket {
    /// Connects to the nl80211 netlink socket.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The netlink socket connection fails
    /// - The nl80211 family cannot be resolved
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use netlink_wi::NlSocket;
    /// # fn example() -> netlink_wi::NlResult<()> {
    /// let socket = NlSocket::connect()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn connect() -> Result<Self> {
        let (socket, _) = NlRouter::connect(NlFamily::Generic, None, Groups::empty())?;
        let nl_type = socket.resolve_genl_family("nl80211")?;
        Ok(Self { socket, nl_type })
    }

    /// Lists all wireless network interfaces currently available on the system.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use netlink_wi::NlSocket;
    /// # fn example() -> netlink_wi::NlResult<()> {
    /// let socket = NlSocket::connect()?;
    /// let interfaces = socket.list_interfaces()?;
    /// for interface in interfaces {
    ///     println!("Interface: {}", interface.interface_index);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_interfaces(&self) -> Result<Vec<WirelessInterface>> {
        let request = Nl80211Request::list_interfaces();
        let recv = self.send(request)?;

        let mut responses = Vec::new();
        Self::handle_dump_response(recv, |handle| {
            responses.push(TryInto::<WirelessInterface>::try_into(handle)?);
            Ok(())
        })?;
        Ok(responses)
    }

    /// Retrieves information about the wireless interface with the specified interface index.
    ///
    /// # Arguments
    ///
    /// * `if_index` - The interface index to query
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use netlink_wi::NlSocket;
    /// # fn example() -> netlink_wi::NlResult<()> {
    /// let socket = NlSocket::connect()?;
    /// if let Some(interface) = socket.get_interface(3)? {
    ///     println!("Found interface: {}", interface.interface_index);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_interface(&self, if_index: u32) -> Result<Option<WirelessInterface>> {
        let request = Nl80211Request::get_interface(if_index);
        let recv = self.send(request)?;

        let mut result: Option<WirelessInterface> = None;
        Self::handle_dump_response(recv, |handle| {
            let device: WirelessInterface = handle.try_into()?;
            if device.interface_index == if_index {
                result = Some(device);
            }
            Ok(())
        })?;
        Ok(result)
    }

    /// Sets the type of a wireless interface.
    ///
    /// Changes the operating mode of the specified wireless interface
    /// (e.g., station, access point, monitor mode).
    ///
    /// # Arguments
    ///
    /// * `if_index` - The interface index to modify
    /// * `if_type` - The new interface type to set
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use netlink_wi::NlSocket;
    /// # use netlink_wi::interface::InterfaceType;
    /// # fn example() -> netlink_wi::NlResult<()> {
    /// let socket = NlSocket::connect()?;
    /// socket.set_interface(3, InterfaceType::Monitor)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_interface(&self, if_index: u32, if_type: InterfaceType) -> Result<()> {
        let request = Nl80211Request::set_interface(if_index, if_type);
        let recv = self.send(request)?;
        Self::handle_ack_response(recv)
    }

    /// Configures the monitor mode flags for an interface in monitor mode.
    ///
    /// # Arguments
    ///
    /// * `if_index` - The interface index to modify
    /// * `flags` - A vector of monitor flags to set
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use netlink_wi::{NlSocket, MonitorFlags};
    /// # fn example() -> netlink_wi::NlResult<()> {
    /// let socket = NlSocket::connect()?;
    /// let flags = vec![MonitorFlags::FcsFail, MonitorFlags::Control];
    /// socket.set_monitor_flags(3, flags)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_monitor_flags(&self, if_index: u32, flags: Vec<MonitorFlags>) -> Result<()> {
        let request = Nl80211Request::set_monitor_flags(if_index, flags);
        let recv = self.send(request)?;
        Self::handle_ack_response(recv)
    }

    /// Sets the channel configuration for a wireless interface.
    ///
    /// # Arguments
    ///
    /// * `config` - The channel configuration to apply
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use netlink_wi::{NlSocket, ChannelConfig};
    /// # use netlink_wi::interface::ChannelWidth;
    /// # fn example() -> netlink_wi::NlResult<()> {
    /// let socket = NlSocket::connect()?;
    /// let config = ChannelConfig::new(
    ///     3, // interface index
    ///     2437, // Channel 6
    ///     ChannelWidth::Width20,
    /// );
    /// socket.set_channel(config)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_channel(&self, config: ChannelConfig) -> Result<()> {
        let request = Nl80211Request::set_channel(config);
        let recv = self.send(request)?;
        Self::handle_ack_response(recv)
    }

    /// Lists all wireless stations associated with an interface.
    ///
    /// Retrieves information about all stations (clients) currently associated
    /// with the specified wireless interface.
    ///
    /// # Arguments
    ///
    /// * `if_index` - The interface index to query
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use netlink_wi::NlSocket;
    /// # fn example() -> netlink_wi::NlResult<()> {
    /// let socket = NlSocket::connect()?;
    /// let stations = socket.list_stations(3)?;
    /// for station in stations {
    ///     println!("Station MAC: {:?}", station.mac);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_stations(&self, if_index: u32) -> Result<Vec<WirelessStation>> {
        let request = Nl80211Request::list_stations(if_index);
        let recv = self.send(request)?;

        let mut responses = Vec::new();
        Self::handle_dump_response(recv, |handle| {
            responses.push(TryInto::<WirelessStation>::try_into(handle)?);
            Ok(())
        })?;
        Ok(responses)
    }

    /// Lists all physical wireless devices (wiphys).
    ///
    /// Retrieves information about all physical wireless devices (PHYs) available
    /// on the system. Each physical device may have multiple virtual interfaces.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use netlink_wi::NlSocket;
    /// # fn example() -> netlink_wi::NlResult<()> {
    /// let socket = NlSocket::connect()?;
    /// let devices = socket.list_physical_devices()?;
    /// for device in devices {
    ///     println!("PHY index: {}", device.wiphy_index);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_physical_devices(&self) -> Result<Vec<PhysicalDevice>> {
        let request = Nl80211Request::list_physical_devices();
        let recv = self.send(request)?;

        let mut responses = HashMap::new();
        Self::handle_dump_response(recv, |handle| {
            let device: PhysicalDevice = handle.try_into()?;
            responses
                .entry(device.wiphy_index)
                .and_modify(|d: &mut PhysicalDevice| d.merge(&device))
                .or_insert(device);
            Ok(())
        })?;
        Ok(responses.values().cloned().collect())
    }

    /// Retrieves detailed information about the physical wireless device (PHY)
    /// with the specified wiphy index.
    ///
    /// # Arguments
    ///
    /// * `wiphy_index` - The wiphy index to query
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use netlink_wi::NlSocket;
    /// # fn example() -> netlink_wi::NlResult<()> {
    /// let socket = NlSocket::connect()?;
    /// if let Some(device) = socket.get_physical_device(0)? {
    ///     println!("PHY: {}", device.wiphy_index);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_physical_device(&self, wiphy_index: u32) -> Result<Option<PhysicalDevice>> {
        let request = Nl80211Request::get_physical_device(wiphy_index);
        let recv = self.send(request)?;

        let mut result: Option<PhysicalDevice> = None;
        Self::handle_dump_response(recv, |handle| {
            let device: PhysicalDevice = handle.try_into()?;
            if device.wiphy_index == wiphy_index {
                if let Some(d) = result.as_mut() {
                    d.merge(&device);
                } else {
                    result = Some(device);
                }
            }
            Ok(())
        })?;
        Ok(result)
    }

    /// Retrieves information about the current wireless regulatory domain,
    /// which defines the allowed frequencies, power limits, and other
    /// regulatory restrictions for wireless operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use netlink_wi::NlSocket;
    /// # fn example() -> netlink_wi::NlResult<()> {
    /// let socket = NlSocket::connect()?;
    /// let domains = socket.get_regulatory_domain()?;
    /// for domain in domains {
    ///     println!("Regulatory domain: {:?}", domain);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_regulatory_domain(&self) -> Result<Vec<RegulatoryDomain>> {
        let request = Nl80211Request::get_regulatory_domain();
        let recv = self.send(request)?;

        let mut responses = Vec::new();
        Self::handle_dump_response(recv, |handle| {
            responses.push(TryInto::<RegulatoryDomain>::try_into(handle)?);
            Ok(())
        })?;
        Ok(responses)
    }

    /// Triggers a wireless scan on the specified interface.
    ///
    /// # Arguments
    ///
    /// * `if_index` - The interface index to scan with
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use netlink_wi::NlSocket;
    /// # fn example() -> netlink_wi::NlResult<()> {
    /// let socket = NlSocket::connect()?;
    /// socket.trigger_scan(3)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn trigger_scan(&self, if_index: u32) -> Result<()> {
        let request = Nl80211Request::trigger_scan(if_index);
        let recv = self.send(request)?;
        Self::handle_ack_response(recv)
    }

    /// Aborts an ongoing wireless scan on the specified interface.
    ///
    /// # Arguments
    ///
    /// * `if_index` - The interface index to abort the scan on
    ///
    /// # Errors
    ///
    /// Returns NlError ENOENT if a scan is not running.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use netlink_wi::NlSocket;
    /// # fn example() -> netlink_wi::NlResult<()> {
    /// let socket = NlSocket::connect()?;
    /// socket.trigger_scan(3)?;
    /// // some time later
    /// socket.abort_scan(3)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn abort_scan(&self, if_index: u32) -> Result<()> {
        let request = Nl80211Request::abort_scan(if_index);
        let recv = self.send(request)?;
        Self::handle_ack_response(recv)
    }

    fn send(
        &self,
        request: Nl80211Request,
    ) -> std::result::Result<
        NlRouterReceiverHandle<Nlmsg, Neli80211Header>,
        RouterError<u16, Neli80211Header>,
    > {
        if cfg!(debug_assertions) {
            let mut b: Cursor<Vec<u8>> = Cursor::new(Vec::new());
            request.nl_payload.to_bytes(&mut b).unwrap();
            let octets: String = b.get_ref().iter().fold(String::new(), |mut output, b| {
                let _ = write!(output, "{b:02x} ");
                output
            });
            debug!("[PAYLOAD] {octets}");
        }
        self.socket
            .send(self.nl_type, request.nl_flags, request.nl_payload)
    }

    fn handle_dump_response<F: FnMut(&Attrs<'_, Attribute>) -> Result<()>>(
        recv: NlRouterReceiverHandle<Nlmsg, Neli80211Header>,
        mut f: F,
    ) -> Result<()> {
        for response in recv {
            let response = response?;
            match response.nl_payload() {
                NlPayload::Err(err) => {
                    debug!("Error when reading dump response: {err}");
                    return Err(err.clone().into());
                }
                NlPayload::Payload(payload) => {
                    let handle = payload.attrs().get_attr_handle();
                    f(&handle)?
                }
                NlPayload::Empty | NlPayload::Ack(_) | NlPayload::DumpExtAck(_) => (),
            };
        }
        Ok(())
    }

    fn handle_ack_response(recv: NlRouterReceiverHandle<Nlmsg, Neli80211Header>) -> Result<()> {
        for response in recv {
            let response = response?;
            match response.nl_payload() {
                NlPayload::Err(err) => {
                    debug!("Error when reading ack response: {err}");
                    return Err(err.clone().into());
                }
                NlPayload::Payload(_)
                | NlPayload::Empty
                | NlPayload::Ack(_)
                | NlPayload::DumpExtAck(_) => (),
            };
        }
        Ok(())
    }
}
