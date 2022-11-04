use deku::prelude::*;

use mud::packet::header::DnsHeader;

fn main() {

	let data: Vec<u8> = vec![

		0b0000_0101,
		0b0011_1001,	// ID

		0b1000_0000 |	// QR
		0b0000_0000 |	// OPCODE
		0b0000_0000 |	// AA
		0b0000_0000 |	// TC
		0b0000_0001,	// RD

		0b0000_0000 |	// RA
		0b0000_0000 |	// Z
		0b0000_0000,	// RCODE

		0,			// QDCOUNT
		1,			// QDCOUNT
		0,			// ANCOUNT
		0,			// ANCOUNT
		0,			// NSCOUNT
		0,			// NSCOUNT
		0,			// ARCOUNT
		0,			// ARCOUNT
	];

	let (_res, val) = DnsHeader::from_bytes((data.as_ref(), 0)).unwrap();


	println!("{:?}", val);
}
