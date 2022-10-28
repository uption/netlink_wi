use netlink_wi::NlSocket;

fn main() {
    let mut socket = NlSocket::connect().unwrap();
    let interfaces = socket.list_interfaces().unwrap();
    for interface in interfaces {
        let interface = interface;
        let stations = socket.list_stations(interface.interface_index).unwrap();
        for station in stations {
            let station = station;
            println!("{:#?}", station);
        }
    }
}
