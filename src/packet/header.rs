use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct DnsHeader {

	pub id: u16,

	#[deku(bits = "1")]
	pub qr: u8,

	#[deku(bits = "4")]
	pub opcode: u8,

	#[deku(bits = "1")]
	pub aa: u8,
	#[deku(bits = "1")]
	pub tc: u8,
	#[deku(bits = "1")]
	pub rd: u8,
	#[deku(bits = "1")]
	pub ra: u8,

	#[deku(bits = "3")]
	pub z: u8,
	#[deku(bits = "4")]
	pub rcode: u8,

	pub qd_count: u16,
	pub an_count: u16,
	pub ns_count: u16,
	pub ar_count: u16,
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn it_creates_a_header_from_bytes() {

		let data: Vec<u8> = vec![

			0b0000_0101,
			0b0011_1001,	// ID

			0b1000_0001,	// QR|OPCODE|AA|TC|RD

			0b0000_0000,	// RA|Z|RCODE

			0, 1,	// QDCOUNT
			0, 0,	// ANCOUNT
			0, 0,	// NSCOUNT
			0, 0,	// ARCOUNT
		];

		let (_res, header) = DnsHeader::from_bytes(
			(data.as_ref(), 0)
		).unwrap();

		assert_eq!(header, DnsHeader {
			id: 1337,

			qr: 0b1,
			opcode: 0b0000,
			aa: 0b0,
			tc: 0b0,
			rd: 0b1,

			ra: 0b0,
			z: 0b000,
			rcode: 0b0000,

			qd_count: 1,
			an_count: 0,
			ns_count: 0,
			ar_count: 0,
		});
	}
}
