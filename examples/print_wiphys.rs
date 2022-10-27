/// Print physical wireless device information.
use netlink_wi::NlSocket;

fn main() {
    let socket = NlSocket::connect().unwrap();
    let devices = socket.list_physical_devices().unwrap();
    for device in devices {
        let device = device.unwrap();
        println!("{:#?}", device);
    }
}
