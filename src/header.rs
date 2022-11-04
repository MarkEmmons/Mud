use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct DnsHeader {

	id: u16,

	#[deku(bits = "1")]
	qr: u8,

	#[deku(bits = "4")]
	opcode: u8,

	#[deku(bits = "1")]
	aa: u8,
	#[deku(bits = "1")]
	tc: u8,
	#[deku(bits = "1")]
	rd: u8,
	#[deku(bits = "1")]
	ra: u8,

	#[deku(bits = "3")]
	z: u8,
	#[deku(bits = "4")]
	rcode: u8,

	qd_count: u16,
	an_count: u16,
	ns_count: u16,
	ar_count: u16,
}
