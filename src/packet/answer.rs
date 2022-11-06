use deku::prelude::*;

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, PartialEq)]
#[deku(endian = "big")]
pub struct DnsAnswer {

	#[deku(temp)]
	_name_length: u8,

	#[deku(bytes_read = "_name_length")]
	pub name: Vec<u8>,

	pub atype: u16,

	pub class: u16,

	pub ttl: u32,

	pub rdlength: u16,

	#[deku(bytes_read = "rdlength")]
	pub rdata: Vec<u8>,
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn it_creates_an_answer_with_the_correct_rdata() {

		let data: Vec<u8> = vec![

			// _name_length = 3
			0b0000_0011,

			// rdata
			0b1000_0000,
			0b0100_0000,
			0b0010_0000,

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

		let ((rest, offset), packet) = DnsAnswer::from_bytes(
			(data.as_ref(), 0)
		).unwrap();

		println!("{:?}", packet);

		assert_eq!(rest.len(), 0);
		assert_eq!(offset, 1);

		let dater: Vec<u8> = vec![

			// rdata
			0b1000_0000,
			0b0100_0000,
			0b0010_0000,

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

		assert_eq!(packet.to_bytes().unwrap(), dater);
	}
}
