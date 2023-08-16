use log::LevelFilter;
use netlink_wi::AsyncNlSocket;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .with_module_level("neli", LevelFilter::Info)
        .init()
        .unwrap();

    let mut socket = AsyncNlSocket::connect().await.unwrap();
    let interfaces = socket.list_interfaces().await.unwrap();
    for interface in interfaces {
        println!("{:#?}", interface);
    }
}
