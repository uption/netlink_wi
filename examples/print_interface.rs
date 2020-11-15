use netlink_wi::NlSocket;

fn main() {
    let socket = NlSocket::connect().unwrap();
    let interfaces = socket.list_interfaces().unwrap();
    for interface in interfaces {
        let interface = interface.unwrap();
        println!("{:?}", interface);
    }
}
