use log::LevelFilter;
#[cfg(feature = "async")]
use netlink_wi::AsyncNlSocket;
#[cfg(not(feature = "async"))]
use netlink_wi::NlSocket;
use simple_logger::SimpleLogger;

#[cfg(not(feature = "async"))]
fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .with_module_level("neli", LevelFilter::Info)
        .init()
        .unwrap();

    let mut socket = NlSocket::connect().unwrap();
    let interfaces = socket.list_interfaces().unwrap();
    for interface in interfaces {
        println!("{interface:#?}");
    }
}

#[cfg(feature = "async")]
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
        println!("{interface:#?}");
    }
}
