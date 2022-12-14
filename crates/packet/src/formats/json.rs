use crate::DnsPacket;

#[cfg(feature = "json")]
pub fn print_json(packet: &DnsPacket) {

	let serialized = serde_json::to_string(packet).unwrap();

	println!("{}", serialized);
}
