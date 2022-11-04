use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct DnsQuestion {

	#[deku(update = "self.qname.len()")]
	count: u8,

	#[deku(count = "count")]
	qname: Vec<u8>,

	qtype: u16,

	qclass: u16,
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
