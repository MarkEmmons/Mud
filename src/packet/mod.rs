use deku::prelude::*;

use header::DnsHeader;
use question::DnsQuestion;

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
