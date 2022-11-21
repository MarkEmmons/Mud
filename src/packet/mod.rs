use deku::prelude::*;
use deku::bitvec::{BitSlice, Msb0};
use serde::{Serialize, Deserialize};
use tracing::{event, Level};

use header::DnsHeader;
use question::DnsQuestion;
use resource::DnsResource;

use crate::opts::MudOpts;

pub mod header;
pub mod question;
pub mod resource;

//pub mod name;

#[derive(Debug, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct DnsPacket {

	pub header: DnsHeader,

	#[deku(count = "header.qd_count")]
	pub questions: Vec<DnsQuestion>,

	#[deku(count = "header.an_count")]
	pub answers: Vec<DnsResource>,

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

				let offset: u16 =
					((label_length as u16) << 8) + label_byte as u16;
				//& 0x3FFF;

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

	pub fn print_response(&self) {

		// TEMP
		let serialized = serde_yaml::to_string(self).unwrap();
		println!("serialized = {}", serialized);

		let deserialized: DnsPacket = serde_yaml::from_str(&serialized).unwrap();
		println!("deserialized = {:?}", deserialized);

		println!("; <<>> Mud 0.0.1 <<>> TODO");
		println!(";; global options: TODO");

		println!(";; Got answer:");
		self.header.print();

		for q in self.questions.iter() { q.print(); }
		for a in self.answers.iter() { a.print(); }
		for a in self.authority.iter() { a.print(); }
		for a in self.additional.iter() { a.print(); }

		println!(";; STATS - TODO");
		println!("");
	}
}

// Static methods
impl DnsPacket {

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
			questions: vec![DnsQuestion {
				qname: DnsQuestion::encode_domain(&opts.name),
				qtype: 0x0001,
				qclass: 0x0001,
			}],
			answers: Vec::new(),
			authority: Vec::new(),
			additional: Vec::new(),
		}
	}

	#[cfg(test)]
	pub fn load_test_yaml(_file: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

		//let mut file = std::fs::File::open(file)?;
		//let mut yaml = String::new();
		//file.read_to_string(&mut yaml)?;

		//let de = serde_yaml::Deserializer::from_str(&yaml);

		//println!("{:?}", de);

		Ok(vec![u8::MIN])
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn it_loads_a_yaml() {

		let yaml = "tests/resources/packets/simple-question-answer.yml";

		match DnsPacket::load_test_yaml(yaml) {

			Ok(vecky) => assert_eq!(vecky[0], 0),
			Err(e) => panic!("{}", e),
		}
	}

	#[test]
	fn it_creates_a_packet() {

		let data: Vec<u8> = vec![

			//// Header
			0x01, 0x46, 0x81, 0x80, 0x00, 0x01, 0x00, 0x01,
			0x00, 0x00, 0x00, 0x00,

			//// Question
			// www
			0x03, 0x77, 0x77, 0x77,
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
