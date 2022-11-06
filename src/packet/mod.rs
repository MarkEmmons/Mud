use deku::prelude::*;
use deku::bitvec::{BitSlice, Msb0};

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
	//pub answer: DnsAnswer,
	//pub authority: DnsAuthority,
	//pub additional: DnsAdditional,
}

impl DnsPacket {

	pub fn read_name(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, Vec<u8>), DekuError> {

		let mut name: Vec<u8> = Vec::new();

		let (mut remainder, mut label_length) = u8::read(rest, ())?;

		let mut label_byte: u8;
		name.push(label_length);

		while label_length > 0 {

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
			}
		}
	}

	pub fn print_response(&self) {

		println!("{:?}", self);
	}
}
