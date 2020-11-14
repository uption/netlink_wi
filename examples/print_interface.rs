use netlink_wi::NlSocket;

fn main() {
    let socket = NlSocket::connect();
    let interfaces = socket.list_interfaces();
    for interface in interfaces {
        println!("{:?}", interface);
        println!("{}", interface.mac.unwrap());
        println!("{}", interface.channel_width.unwrap());
    }
}
