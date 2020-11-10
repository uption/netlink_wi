use netlink_wi::NlSocket;

fn main() {
    let socket = NlSocket::connect();
    let interfaces = socket.list_interfaces();
    println!("{:?}", interfaces);
}
