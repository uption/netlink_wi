/// Print physical wireless device information.
use log::LevelFilter;
use netlink_wi::NlSocket;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .with_module_level("neli", LevelFilter::Info)
        .init()
        .unwrap();

    let mut socket = NlSocket::connect().unwrap();
    let devices = socket.list_physical_devices().unwrap();
    for device in devices {
        println!("{device:#?}");
    }
}
