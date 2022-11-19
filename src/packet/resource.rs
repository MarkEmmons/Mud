use deku::prelude::*;

use crate::packet::DnsPacket;

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, PartialEq)]
#[deku(endian = "big")]
pub struct DnsResource {

	#[deku(reader = "DnsPacket::read_name(deku::rest)")]
	pub name: Vec<u8>,

	pub atype: u16,

	pub class: u16,

	pub ttl: u32,

	pub rdlength: u16,

	#[deku(bytes_read = "rdlength")]
	pub rdata: Vec<u8>,
}

impl DnsResource {

	pub fn print(&self) {

		println!(";; ANSWER SECTION:");

		println!("{:?}", self.rdata);

		println!("");
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn it_creates_a_resource_with_the_correct_rdata_full() {

		let data: Vec<u8> = vec![

			// first label = 3
			0b0000_0011,
			0b0000_1000,
			0b0000_1000,
			0b0000_1000,

			// second label = 5
			0b0000_0101,
			0b0001_0000,
			0b0001_0000,
			0b0001_0000,
			0b0001_0000,
			0b0001_0000,

			// null byte
			0b0000_0000,

			// Atype
			0b0000_0000,
			0b0000_0001,

			// Class
			0b0000_0000,
			0b0000_0001,

			// TTL
			0b0000_0000,
			0b0000_0000,
			0b0000_0000,
			0b0000_0000,

			// rdlength = 5
			0b0000_0000,
			0b0000_0101,

			// rdata
			0b0000_0001,
			0b0000_0010,
			0b0000_0100,
			0b0000_1000,
			0b0001_0000,
		];

		let ((rest, offset), packet) = DnsResource::from_bytes(
			(data.as_ref(), 0)
		).unwrap();

		println!("{:?}", packet);

		assert_eq!(rest.len(), 0);
		assert_eq!(offset, 0);
	}

	#[test]
	fn it_creates_a_resource_with_the_correct_rdata_compressed() {

		let data: Vec<u8> = vec![

			// Pointer/Offset
			0b1100_0000,
			0b0000_1100,

			// Atype
			0b0000_0000,
			0b0000_0001,

			// Class
			0b0000_0000,
			0b0000_0001,

			// TTL
			0b0000_0000,
			0b0000_0000,
			0b0000_0010,
			0b0101_0100,

			// rdlength = 4
			0b0000_0000,
			0b0000_0100,

			// rdata
			0b0101_1111,
			0b1101_1001,
			0b1010_0011,
			0b1111_0110,
		];

		let ((rest, offset), packet) = DnsResource::from_bytes(
			(data.as_ref(), 0)
		).unwrap();

		println!("{:?}", packet);
		assert_eq!(packet.rdlength, 4);
		assert_eq!(packet.rdata.len(), 4);

		assert_eq!(rest.len(), 0);
		assert_eq!(offset, 0);
	}
}
