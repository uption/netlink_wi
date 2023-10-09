use std::collections::HashMap;
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
use crate::interface::{ChannelWidth, InterfaceType};
use crate::netlink::{Neli80211Header, Nl80211Request};
use crate::reg_domain::RegulatoryDomain;
use crate::station::WirelessStation;
use crate::wiphy::PhysicalDevice;

use super::interface::WirelessInterface;

/// Netlink socket.
pub struct NlSocket {
    socket: NlRouter,
    nl_type: u16,
}

impl NlSocket {
    /// Connect netlink socket.
    pub fn connect() -> Result<Self> {
        let (socket, _) = NlRouter::connect(NlFamily::Generic, None, Groups::empty())?;
        let nl_type = socket.resolve_genl_family("nl80211")?;
        Ok(Self { socket, nl_type })
    }

    pub fn list_interfaces(&mut self) -> Result<Vec<WirelessInterface>> {
        let request = Nl80211Request::list_interfaces();
        let recv = self.send(request)?;

        let mut responses = Vec::new();
        Self::handle_dump_response(recv, |handle| {
            responses.push(TryInto::<WirelessInterface>::try_into(handle)?);
            Ok(())
        })?;
        Ok(responses)
    }

    pub fn set_interface(&mut self, if_index: u32, if_type: InterfaceType) -> Result<()> {
        let request = Nl80211Request::set_interface(if_index, if_type);
        let recv = self.send(request)?;
        Self::handle_ack_response(recv)
    }

    pub fn set_monitor_flags(&mut self, if_index: u32, flags: Vec<MonitorFlags>) -> Result<()> {
        let request = Nl80211Request::set_monitor_flags(if_index, flags);
        let recv = self.send(request)?;
        Self::handle_ack_response(recv)
    }

    pub fn set_channel(&mut self, if_index: u32, freq: u32, width: ChannelWidth) -> Result<()> {
        let request = Nl80211Request::set_channel(if_index, freq, width);
        let recv = self.send(request)?;
        Self::handle_ack_response(recv)
    }

    pub fn list_stations(&mut self, if_index: u32) -> Result<Vec<WirelessStation>> {
        let request = Nl80211Request::list_stations(if_index);
        let recv = self.send(request)?;

        let mut responses = Vec::new();
        Self::handle_dump_response(recv, |handle| {
            responses.push(TryInto::<WirelessStation>::try_into(handle)?);
            Ok(())
        })?;
        Ok(responses)
    }

    pub fn list_physical_devices(&mut self) -> Result<Vec<PhysicalDevice>> {
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

    pub fn get_physical_device(&mut self, wiphy_index: u32) -> Result<Option<PhysicalDevice>> {
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

    pub fn get_regulatory_domain(&mut self) -> Result<Vec<RegulatoryDomain>> {
        let request = Nl80211Request::get_regulatory_domain();
        let recv = self.send(request)?;

        let mut responses = Vec::new();
        Self::handle_dump_response(recv, |handle| {
            responses.push(TryInto::<RegulatoryDomain>::try_into(handle)?);
            Ok(())
        })?;
        Ok(responses)
    }

    /// Trigger a new scan.
    pub fn trigger_scan(&mut self, if_index: u32) -> Result<()> {
        let request = Nl80211Request::trigger_scan(if_index);
        let recv = self.send(request)?;
        Self::handle_ack_response(recv)
    }

    /// Stop an ongoing scan.
    ///
    /// Returns NlError ENOENT if a scan is not running.
    pub fn abort_scan(&mut self, if_index: u32) -> Result<()> {
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
            let octets: String = b.get_ref().iter().map(|v| format!("{:02x} ", v)).collect();
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
                NlPayload::Empty | NlPayload::Ack(_) => (),
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
                NlPayload::Payload(_) | NlPayload::Empty | NlPayload::Ack(_) => (),
            };
        }
        Ok(())
    }
}
