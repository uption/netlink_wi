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

    let socket = NlSocket::connect().unwrap();
    loop {
        let domains = socket.get_regulatory_domain().unwrap();
        for domain in domains {
            println!("{domain:#?}");
        }
    }
}
