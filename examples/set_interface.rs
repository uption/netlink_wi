use log::LevelFilter;
use netlink_wi::{interface::InterfaceType, NlSocket};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .with_module_level("neli", LevelFilter::Info)
        .init()
        .unwrap();

    let mut socket = NlSocket::connect().unwrap();
    let interfaces: Vec<_> = socket
        .list_interfaces()
        .unwrap()
        .into_iter()
        .filter(|i| !i.name.is_empty())
        .collect();
    if let Some(i) = interfaces.first() {
        socket
            .set_interface(i.interface_index, InterfaceType::Monitor)
            .unwrap();
    } else {
        println!("No interfaces found")
    }
}
