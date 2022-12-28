use crate::DnsPacket;

#[cfg(feature = "yaml")]
pub fn print_yaml(packet: &DnsPacket) {

	let serialized = serde_yaml::to_string(packet).unwrap();

	println!("{}", serialized);
}
