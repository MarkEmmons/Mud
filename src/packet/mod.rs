use deku::prelude::*;

use header::DnsHeader;
use question::{DnsQuestion, encode_domain};

use crate::opts::MudOpts;

pub mod header;
pub mod question;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct DnsPacket {

	pub header: DnsHeader,
	pub question: DnsQuestion,
	//answer: DnsAnswer,
	//authority: DnsAuthority,
	//additional: DnsAdditional,
}

impl DnsPacket {

	pub fn new_question(opts: MudOpts) -> DnsPacket {

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
}
