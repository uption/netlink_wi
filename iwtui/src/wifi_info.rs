use netlink_wi::interface::{InterfaceType, WirelessInterface};
use netlink_wi::station::WirelessStation;
use netlink_wi::wiphy::PhysicalDevice;
use netlink_wi::NlSocket;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

// Define messages our actor will handle
enum Message {
    GetInterfaces(Sender<Vec<WirelessInterface>>),
    UpdateInterfaces(Vec<WirelessInterface>),
    GetDevices(Sender<Vec<PhysicalDevice>>),
    UpdateDevices(Vec<PhysicalDevice>),
    GetStations(Sender<Vec<WirelessStation>>),
    UpdateStations(Vec<WirelessStation>),
    Shutdown,
}

#[derive(Debug)]
pub struct WifiInfoWorker {
    sender: Sender<Message>,
    update_thread: Option<thread::JoinHandle<()>>,
    actor_thread: Option<thread::JoinHandle<()>>,
}

impl WifiInfoWorker {
    pub fn start() -> Self {
        let (sender, receiver) = channel();

        let actor_thread = thread::spawn(move || {
            WifiInfoActor::run(receiver);
        });

        let update_sender = sender.clone();
        let update_thread = thread::spawn(move || {
            let nl_socket = NlSocket::connect().unwrap();

            loop {
                match nl_socket.list_interfaces() {
                    Ok(interface_list) => {
                        for interface in &interface_list {
                            if interface.interface_type != Some(InterfaceType::Station) {
                                continue;
                            }
                            match nl_socket.list_stations(interface.interface_index) {
                                Ok(station_list) => {
                                    if update_sender
                                        .send(Message::UpdateStations(station_list))
                                        .is_err()
                                    {
                                        // Actor has terminated
                                        break;
                                    }
                                }
                                Err(e) => eprintln!("Failed to list stations: {e}"),
                            }
                        }

                        if update_sender
                            .send(Message::UpdateInterfaces(interface_list))
                            .is_err()
                        {
                            // Actor has terminated
                            break;
                        }
                    }
                    Err(e) => eprintln!("Failed to list interfaces: {e}"),
                }
                match nl_socket.list_physical_devices() {
                    Ok(device_list) => {
                        if update_sender
                            .send(Message::UpdateDevices(device_list))
                            .is_err()
                        {
                            // Actor has terminated
                            break;
                        }
                    }
                    Err(e) => eprintln!("Failed to list devices: {e}"),
                }

                thread::sleep(Duration::from_millis(100));
            }
        });

        Self {
            sender,
            update_thread: Some(update_thread),
            actor_thread: Some(actor_thread),
        }
    }

    pub fn interfaces(&self) -> Vec<WirelessInterface> {
        self.request(|tx| Message::GetInterfaces(tx))
    }

    pub fn devices(&self) -> Vec<PhysicalDevice> {
        self.request(|tx| Message::GetDevices(tx))
    }

    pub fn stations(&self) -> Vec<WirelessStation> {
        self.request(|tx| Message::GetStations(tx))
    }

    fn request<T, F>(&self, f: F) -> T
    where
        F: FnOnce(Sender<T>) -> Message,
    {
        let (response_tx, response_rx) = channel();
        self.sender
            .send(f(response_tx))
            .expect("Actor thread has terminated");

        // Wait for response
        response_rx.recv().expect("Actor thread has terminated")
    }
}

impl Drop for WifiInfoWorker {
    fn drop(&mut self) {
        let _ = self.sender.send(Message::Shutdown);

        if let Some(handle) = self.actor_thread.take() {
            let _ = handle.join();
        }

        if let Some(handle) = self.update_thread.take() {
            let _ = handle.join();
        }
    }
}

struct WifiInfoActor {
    interfaces: Vec<WirelessInterface>,
    devices: Vec<PhysicalDevice>,
    stations: Vec<WirelessStation>,
}

impl WifiInfoActor {
    fn run(receiver: Receiver<Message>) {
        let mut actor = WifiInfoActor {
            interfaces: Vec::new(),
            devices: Vec::new(),
            stations: Vec::new(),
        };

        while let Ok(message) = receiver.recv() {
            match message {
                Message::GetInterfaces(response_channel) => {
                    let _ = response_channel.send(actor.interfaces.clone());
                }
                Message::UpdateInterfaces(new_interfaces) => {
                    actor.interfaces = new_interfaces;
                }
                Message::GetDevices(response_channel) => {
                    let _ = response_channel.send(actor.devices.clone());
                }
                Message::UpdateDevices(new_devices) => {
                    actor.devices = new_devices;
                }
                Message::GetStations(response_channel) => {
                    let _ = response_channel.send(actor.stations.clone());
                }
                Message::UpdateStations(new_stations) => {
                    actor.stations = new_stations;
                }
                Message::Shutdown => {
                    break;
                }
            }
        }
    }
}
