use clap::Parser;

use mud::args::MudOpts;
use mud::packet::{
	DnsPacket,
	header::DnsHeader,
	question::{DnsQuestion, encode_domain}
};

fn main() {

	let args = MudOpts::parse();

	let req = DnsPacket {
		header: DnsHeader {
			id: 1337,
			qr: 0b1,
			opcode: 0b0000,
			aa: 0b0,
			tc: 0b0,
			rd: 0b1,
			ra: 0b0,
			z: 0b000,
			rcode: 0b0000,
			qd_count: 1,
			an_count: 0,
			ns_count: 0,
			ar_count: 0,
		},
		question: DnsQuestion {
			count: 0,
			qname: encode_domain(&args.name),
			qtype: 0x0001,
			qclass: 0x0001,
		}
	};

	println!("Server: {}", args.server);
	println!("Name: {}", args.name);
	println!("Type: {}", args.query_type);
	println!("");
	println!("Request: {:?}", req);
}
