use deku::prelude::*;
use deku::bitvec::{BitSlice, Msb0};
use tracing::{event, Level};

use header::DnsHeader;
use question::{DnsQuestion, encode_domain};
use answer::DnsAnswer;

use crate::opts::MudOpts;

pub mod header;
pub mod question;
pub mod answer;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct DnsPacket {

	pub header: DnsHeader,
	pub question: DnsQuestion,
	pub answer: DnsAnswer,
	//pub authority: DnsAuthority,
	//pub additional: DnsAdditional,
}

impl DnsPacket {

	// See RFC 1035 - Section 4 for instructions on how domain names are encoded
	pub fn read_name(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, Vec<u8>), DekuError> {

		let mut label_byte: u8;
		let mut name: Vec<u8> = Vec::new();

		let (mut remainder, mut label_length) = u8::read(rest, ())?;
		name.push(label_length);

		// FIX: Pointers can come after labels
		// Initial octet indicates a pointer, not a label
		if label_length >= 0xC {

			(remainder, label_byte) = u8::read(remainder, ())?;
			name.push(label_byte);

			let offset: u16 = (
				((label_length as u16) << 8) + label_byte as u16
			) & 0x3FFF;

			// TODO: Use an enum of Vec<u8>, *ptr to domain name
			event!(Level::INFO, "Domain name at offset={}", offset);

		} else {

			while label_length > 0 {

				for _ in 0..label_length {

					(remainder, label_byte) = u8::read(remainder, ())?;
					name.push(label_byte);
				}

				(remainder, label_length) = u8::read(remainder, ())?;
				name.push(label_length);
			}
		}

		Ok((remainder, name))
	}

	pub fn new_question(opts: &MudOpts) -> DnsPacket {

		const ID: u16 = 326;

		// assert(opts.name)

		DnsPacket {
			header: DnsHeader {
				id: ID,
				qr: 0b0,
				opcode: 0b0000,
				aa: 0b0,
				tc: 0b0,
				rd: 0b1,
				ra: 0b0,
				z: 0b010,
				rcode: 0b0000,
				qd_count: 1,
				an_count: 0,
				ns_count: 0,
				ar_count: 0,
			},
			question: DnsQuestion {
				qname: encode_domain(&opts.name),
				qtype: 0x0001,
				qclass: 0x0001,
			},
			answer: DnsAnswer {
				name: vec![0x0],
				atype: 0x00,
				class: 0x00,
				ttl: 0x0000,
				rdlength: 0x00,
				rdata: vec![0x0],
			}
		}
	}

	pub fn print_response(&self) {

		println!("{:?}", self);

		//let path = Path::new("response");
		//let mut file = match File::create(&path) {
		//	Err(e) => panic!("Could not create {}: {}", path.display(), e),
		//	Ok(file) => file,
		//};

		//match file.write_all(&res[..number_of_bytes]) {
		//	Err(e) => panic!("Couldn't write to {}: {}", path.display(), e),
		//	Ok(_) => event!(Level::INFO, "Sending buffer..."),
		//}
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn it_creates_a_packet() {

		let data: Vec<u8> = vec![

			//// HEADER
			0b0000_0001,
			0b0100_0110,
			0b1000_0001,
			0b1000_0000,
			0b0000_0000,
			0b0000_0001,
			0b0000_0000,
			0b0000_0001,
			0b0000_0000,
			0b0000_0000,
			0b0000_0000,
			0b0000_0000,

			//// QUESTION
			// www
			0b0000_0011,
			0b0111_0111,
			0b0111_0111,
			0b0111_0111,

			// archlinux
			0b0000_1001,
			0b0110_0001,
			0b0111_0010,
			0b0110_0011,
			0b0110_1000,
			0b0110_1100,
			0b0110_1001,
			0b0110_1110,
			0b0111_0101,
			0b0111_1000,

			// org
			0b0000_0011,
			0b0110_1111,
			0b0111_0010,
			0b0110_0111,

			// Null Byte
			0b0000_0000,

			// QType
			0b0000_0000,
			0b0000_0001,

			// QClass
			0b0000_0000,
			0b0000_0001,

			//// ANSWER
			// Pointer to Q-name
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
			0b0000_1010,
			0b0100_0101,

			// rdlength = 4
			0b0000_0000,
			0b0000_0100,

			// rdata
			0b0101_1111,
			0b1101_1001,
			0b1010_0011,
			0b1111_0110,
		];

		let ((rest, offset), packet) = DnsPacket::from_bytes(
			(data.as_ref(), 0)
		).unwrap();

		println!("{:?}", packet);

		assert_eq!(rest.len(), 0);
		assert_eq!(offset, 0);
	}
}
