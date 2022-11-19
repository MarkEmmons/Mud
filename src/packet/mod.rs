use deku::prelude::*;
use deku::bitvec::{BitSlice, Msb0};
use tracing::{event, Level};

use header::DnsHeader;
use question::{DnsQuestion, encode_domain};
use resource::DnsResource;

use crate::opts::MudOpts;

pub mod header;
pub mod question;
pub mod resource;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct DnsPacket {

	pub header: DnsHeader,

	#[deku(count = "header.qd_count")]
	pub question: Vec<DnsQuestion>,

	#[deku(count = "header.an_count")]
	pub answer: Vec<DnsResource>,

	#[deku(count = "header.ns_count")]
	pub authority: Vec<DnsResource>,

	#[deku(count = "header.ar_count")]
	pub additional: Vec<DnsResource>,
}

impl DnsPacket {

	// See RFC 1035 - Section 4 for instructions on how domain names are encoded
	pub fn read_name(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, Vec<u8>), DekuError> {

		let mut label_byte: u8;
		let mut name: Vec<u8> = Vec::new();

		let (mut remainder, mut label_length) = u8::read(rest, ())?;
		name.push(label_length);

		while label_length > 0 {

			// Initial octet indicates a pointer, not a label
			if label_length >= 0xC {

				(remainder, label_byte) = u8::read(remainder, ())?;
				name.push(label_byte);

				let offset: u16 = (
					((label_length as u16) << 8) + label_byte as u16
				) & 0x3FFF;

				// TODO: Use an enum of Vec<u8>, *ptr to domain name
				event!(Level::INFO, "Domain name at offset={}", offset);
				break;
			}

			// Read name from label
			for _ in 0..label_length {

				(remainder, label_byte) = u8::read(remainder, ())?;
				name.push(label_byte);
			}

			(remainder, label_length) = u8::read(remainder, ())?;
			name.push(label_length);
		}

		Ok((remainder, name))
	}

	pub fn new_question(opts: &MudOpts) -> DnsPacket {

		const ID: u16 = 326;

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
			question: vec![DnsQuestion {
				qname: encode_domain(&opts.name),
				qtype: 0x0001,
				qclass: 0x0001,
			}],
			answer: Vec::new(),
			authority: Vec::new(),
			additional: Vec::new(),
		}
	}

	pub fn print_response(&self) {

		println!("; <<>> Mud 0.0.1 <<>> TODO");
		println!(";; global options: TODO");

		println!(";; Got answer:");
		self.header.print();

		for q in self.question.iter() { q.print(); }
		for a in self.answer.iter() { a.print(); }
		for a in self.authority.iter() { a.print(); }
		for a in self.additional.iter() { a.print(); }

		println!(";; TODO: STATS");
		println!("");
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
