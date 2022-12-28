use deku::prelude::*;
use deku::bitvec::{BitSlice, Msb0};
use serde::{Serialize, Deserialize};

use header::DnsHeader;
use question::DnsQuestion;
use resource::DnsResource;

pub extern crate mud_opts as opts;
use opts::MudOpts;

pub mod formats;
pub mod header;
pub mod question;
pub mod resource;

#[cfg(test)]
mod test_packets;

#[derive(
	Debug, PartialEq,
	Serialize, Deserialize,
	DekuRead, DekuWrite
)]
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

	pub fn decode_domain(&self, domain: &Vec<u8>) -> String {

		let mut result = String::new();
		let mut bytes_to_read: u8 = 0;

		for (idx, byte) in domain.iter().enumerate() {

			// Read character and continue
			if bytes_to_read > 0 {

				result.push(*byte as char);
				bytes_to_read -= 1;
				continue;
			}

			// Interpret
			match *byte {

				// Null-byte
				0x00 => break,

				// Pointer to domain
				0xC0..=0xFF => {

					// Offset from start of the packet
					let offset: usize = ((
						((*byte as u16) << 8) + domain[idx+1] as u16
					) & 0x3FFF) as usize;

					return self.decode_domain(
						&self
							.to_bytes()
							.unwrap()[offset..]
							.to_vec()
					);
				},

				// Length of next label
				_ => {
					bytes_to_read = *byte;
					if result.len() > 0 {
						result.push('.');
					}
				},
			} // match *byte
		} // for byte in domain.iter()

		result
	}

	pub fn print_response(&self, message_format: String) {

		// TODO: separate serialize / print
		match &message_format[..] {

			"dig" => formats::dig::print_dig(self),
			"json" => formats::json::print_json(self),
			"yaml" => formats::yaml::print_yaml(self),
			_ => panic!("Invalid message format: {}", message_format),
		}
	}
}

// Static methods
impl DnsPacket {

	// See RFC 1035 - Section 4 for instructions on how domain names are encoded
	pub fn read_name(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, Vec<u8>), DekuError> {

		let mut label_byte: u8;
		let mut name: Vec<u8> = Vec::new();

		let (mut remainder, mut label_length) = u8::read(rest, ())?;
		name.push(label_length);

		while label_length > 0 {

			// Indicates a pointer, not a label
			if label_length >= 0xC {

				(remainder, label_byte) = u8::read(remainder, ())?;
				name.push(label_byte);

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
			questions: vec![DnsQuestion {
				qname: DnsPacket::encode_domain(&opts.name),
				qtype: 0x0001,
				qclass: 0x0001,
			}],
			answers: Vec::new(),
			authority: Vec::new(),
			additional: Vec::new(),
		}
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
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn it_creates_a_packet_with_one_question_and_one_answer() {

		let data = test_packets::simple_answer();

		let ((rest, offset), packet) = DnsPacket::from_bytes(
			(data.as_ref(), 0)
		).unwrap();

		assert_eq!(rest.len(), 0);
		assert_eq!(offset, 0);

		assert_eq!(packet.header.qd_count, 1);
		assert_eq!(packet.header.an_count, 1);
		assert_eq!(packet.header.ns_count, 0);
		assert_eq!(packet.header.ar_count, 0);

		let question_name = &packet.questions[0].qname;
		let answer_name = &packet.answers[0].name;

		assert_eq!(*question_name, vec![0x03, 0x77, 0x77, 0x77, 0x09, 0x61, 0x72, 0x63, 0x68, 0x6C, 0x69, 0x6E, 0x75, 0x78, 0x03, 0x6F, 0x72, 0x67, 0x00]);
		assert_eq!(*answer_name, vec![0xC0, 0x0C]);

		assert_eq!(packet.decode_domain(question_name), "www.archlinux.org");
		assert_eq!(packet.decode_domain(answer_name), "www.archlinux.org");
	}

	#[test]
	fn it_creates_a_packet_with_one_question_and_two_answers() {

		let data = test_packets::double_answer();

		let ((rest, offset), packet) = DnsPacket::from_bytes(
			(data.as_ref(), 0)
		).unwrap();

		assert_eq!(rest.len(), 0);
		assert_eq!(offset, 0);

		assert_eq!(packet.header.qd_count, 1);
		assert_eq!(packet.header.an_count, 2);
		assert_eq!(packet.header.ns_count, 0);
		assert_eq!(packet.header.ar_count, 0);

		let question_name = &packet.questions[0].qname;
		let answer_name1 = &packet.answers[0].name;
		let answer_name2 = &packet.answers[1].name;

		assert_eq!(*question_name, vec![0x03, 0x77, 0x77, 0x77, 0x09, 0x61, 0x72, 0x63, 0x68, 0x6C, 0x69, 0x6E, 0x75, 0x78, 0x03, 0x6F, 0x72, 0x67, 0x00]);
		assert_eq!(*answer_name1, vec![0xC0, 0x0C]);
		assert_eq!(*answer_name2, vec![0xC0, 0x0C]);

		assert_eq!(packet.decode_domain(question_name), "www.archlinux.org");
		assert_eq!(packet.decode_domain(answer_name1), "www.archlinux.org");
		assert_eq!(packet.decode_domain(answer_name2), "www.archlinux.org");
	}

	#[test]
	fn it_creates_a_packet_with_one_answer_and_one_authority() {

		let data = test_packets::one_answer_one_authority();

		let ((rest, offset), packet) = DnsPacket::from_bytes(
			(data.as_ref(), 0)
		).unwrap();

		assert_eq!(rest.len(), 0);
		assert_eq!(offset, 0);

		assert_eq!(packet.header.qd_count, 1);
		assert_eq!(packet.header.an_count, 1);
		assert_eq!(packet.header.ns_count, 1);
		assert_eq!(packet.header.ar_count, 0);

		let question_name = &packet.questions[0].qname;
		let answer_name = &packet.answers[0].name;
		let auth_name = &packet.authority[0].name;

		assert_eq!(*question_name, vec![0x03, 0x77, 0x77, 0x77, 0x09, 0x61, 0x72, 0x63, 0x68, 0x6C, 0x69, 0x6E, 0x75, 0x78, 0x03, 0x6F, 0x72, 0x67, 0x00]);
		assert_eq!(*answer_name, vec![0xC0, 0x0C]);
		assert_eq!(*auth_name, vec![0xC0, 0x0C]);

		assert_eq!(packet.decode_domain(question_name), "www.archlinux.org");
		assert_eq!(packet.decode_domain(answer_name), "www.archlinux.org");
		assert_eq!(packet.decode_domain(auth_name), "www.archlinux.orgA");
	}
}
