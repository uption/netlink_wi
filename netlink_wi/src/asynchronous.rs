use std::collections::HashMap;
use std::fmt::Write;
use std::io::Cursor;

use log::debug;
use neli::consts::nl::Nlmsg;
use neli::consts::socket::NlFamily;
use neli::err::RouterError;
use neli::nl::NlPayload;
use neli::router::asynchronous::{NlRouter, NlRouterReceiverHandle};
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

/// Netlink socket.
pub struct AsyncNlSocket {
    socket: NlRouter,
    nl_type: u16,
}

impl AsyncNlSocket {
    /// Connect netlink socket.
    pub async fn connect() -> Result<Self> {
        let (socket, _) = NlRouter::connect(NlFamily::Generic, None, Groups::empty()).await?;
        let nl_type = socket.resolve_genl_family("nl80211").await?;
        Ok(Self { socket, nl_type })
    }

    pub async fn list_interfaces(&self) -> Result<Vec<WirelessInterface>> {
        let request = Nl80211Request::list_interfaces();
        let recv = self.send(request).await?;
        let mut responses = Vec::new();
        Self::handle_dump_response(recv, |handle| {
            responses.push(TryInto::<WirelessInterface>::try_into(handle)?);
            Ok(())
        })
        .await?;
        Ok(responses)
    }

    pub async fn get_interface(&self, if_index: u32) -> Result<Option<WirelessInterface>> {
        let request = Nl80211Request::get_interface(if_index);
        let recv = self.send(request).await?;

        let mut result: Option<WirelessInterface> = None;
        Self::handle_dump_response(recv, |handle| {
            let device: WirelessInterface = handle.try_into()?;
            if device.interface_index == if_index {
                result = Some(device);
            }
            Ok(())
        })
        .await?;
        Ok(result)
    }

    pub async fn set_interface(&self, if_index: u32, if_type: InterfaceType) -> Result<()> {
        let request = Nl80211Request::set_interface(if_index, if_type);
        let recv = self.send(request).await?;
        Self::handle_ack_response(recv).await
    }

    pub async fn set_monitor_flags(&self, if_index: u32, flags: Vec<MonitorFlags>) -> Result<()> {
        let request = Nl80211Request::set_monitor_flags(if_index, flags);
        let recv = self.send(request).await?;
        Self::handle_ack_response(recv).await
    }

    pub async fn set_channel(&self, config: ChannelConfig) -> Result<()> {
        let request = Nl80211Request::set_channel(config);
        let recv = self.send(request).await?;
        Self::handle_ack_response(recv).await
    }

    pub async fn list_stations(&self, if_index: u32) -> Result<Vec<WirelessStation>> {
        let request = Nl80211Request::list_stations(if_index);
        let recv = self.send(request).await?;

        let mut responses = Vec::new();
        Self::handle_dump_response(recv, |handle| {
            responses.push(TryInto::<WirelessStation>::try_into(handle)?);
            Ok(())
        })
        .await?;
        Ok(responses)
    }

    pub async fn list_physical_devices(&self) -> Result<Vec<PhysicalDevice>> {
        let request = Nl80211Request::list_physical_devices();
        let recv = self.send(request).await?;

        let mut responses = HashMap::new();
        Self::handle_dump_response(recv, |handle| {
            let device: PhysicalDevice = handle.try_into()?;
            responses
                .entry(device.wiphy_index)
                .and_modify(|d: &mut PhysicalDevice| d.merge(&device))
                .or_insert(device);
            Ok(())
        })
        .await?;
        Ok(responses.values().cloned().collect())
    }

    pub async fn get_physical_device(&self, wiphy_index: u32) -> Result<Option<PhysicalDevice>> {
        let request = Nl80211Request::get_physical_device(wiphy_index);
        let recv = self.send(request).await?;

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
        })
        .await?;
        Ok(result)
    }

    pub async fn get_regulatory_domain(&self) -> Result<Vec<RegulatoryDomain>> {
        let request = Nl80211Request::get_regulatory_domain();
        let recv = self.send(request).await?;

        let mut responses = Vec::new();
        Self::handle_dump_response(recv, |handle| {
            responses.push(TryInto::<RegulatoryDomain>::try_into(handle)?);
            Ok(())
        })
        .await?;
        Ok(responses)
    }

    pub async fn trigger_scan(&self, if_index: u32) -> Result<()> {
        let request = Nl80211Request::trigger_scan(if_index);
        let recv = self.send(request).await?;
        Self::handle_ack_response(recv).await
    }

    pub async fn abort_scan(&self, if_index: u32) -> Result<()> {
        let request = Nl80211Request::abort_scan(if_index);
        let recv = self.send(request).await?;
        Self::handle_ack_response(recv).await
    }

    async fn send(
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
            .await
    }

    async fn handle_dump_response<F: FnMut(&Attrs<'_, Attribute>) -> Result<()>>(
        mut recv: NlRouterReceiverHandle<Nlmsg, Neli80211Header>,
        mut f: F,
    ) -> Result<()> {
        while let Some(Ok(response)) = recv.next::<Nlmsg, Neli80211Header>().await {
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

    async fn handle_ack_response(
        mut recv: NlRouterReceiverHandle<Nlmsg, Neli80211Header>,
    ) -> Result<()> {
        while let Some(response) = recv.next::<Nlmsg, Neli80211Header>().await {
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

impl std::fmt::Debug for AsyncNlSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncNlSocket").finish()
    }
}
