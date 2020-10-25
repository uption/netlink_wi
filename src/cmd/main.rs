use netlink_wi::{NlSocket, WirelessInterface};

fn main() {
    let mut socket = NlSocket::connect();
    let interfaces = WirelessInterface::list_interfaces(&mut socket);
    println!("{:?}", interfaces);
}
