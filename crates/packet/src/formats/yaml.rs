use crate::DnsPacket;

pub fn print_yaml(packet: &DnsPacket) {

	let serialized = serde_yaml::to_string(packet).unwrap();

	println!("{}", serialized);
}
