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
    let interfaces = socket.list_interfaces().unwrap();
    for interface in interfaces {
        println!("{:#?}", interface);
    }
}
