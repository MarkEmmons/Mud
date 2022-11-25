use deku::prelude::*;
use serde::{Serialize, Deserialize};

use crate::packet::DnsPacket;

#[deku_derive(DekuRead, DekuWrite)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[deku(endian = "big")]
pub struct DnsQuestion {

	#[deku(reader = "DnsPacket::read_name(deku::rest)")]
	pub qname: Vec<u8>,

	pub qtype: u16,

	pub qclass: u16,
}

impl DnsQuestion {

	pub fn print(&self) {

		println!(";; QUESTION SECTION:");
		println!("");
	}
}
