use deku::prelude::*;

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, PartialEq)]
#[deku(endian = "big")]
pub struct DnsQuestion {

	#[deku(temp)]
	pub count: u8,

	#[deku(count = "count")]
	pub qname: Vec<u8>,

	pub qtype: u16,

	pub qclass: u16,
}

pub fn encode_domain(domain: &str) -> Vec<u8> {

	let mut qname: Vec<u8> = Vec::new();

	for label in domain.split('.') {

		// Label Length
		qname.push(label
			.len()
			.try_into()
			.unwrap()
		);

		// Label Bytes
		for byte in label.bytes() {
			qname.push(byte);
		}
	}

	// Null-label
	qname.push(0x0);

	qname
}
