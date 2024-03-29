use log::LevelFilter;
use netlink_wi::NlSocket;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .with_module_level("neli", LevelFilter::Info)
        .init()
        .unwrap();

    let socket = NlSocket::connect().unwrap();
    let interfaces = socket.list_interfaces().unwrap();
    for interface in interfaces {
        let stations = socket.list_stations(interface.interface_index).unwrap();
        for station in stations {
            println!("{station:#?}");
        }
    }
}
