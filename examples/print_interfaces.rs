use netlink_wi::NlSocket;

fn main() {
    let mut socket = NlSocket::connect().unwrap();
    let interfaces = socket.list_interfaces().unwrap();
    for interface in interfaces {
        let interface = interface;
        println!("{:#?}", interface);
    }
}
