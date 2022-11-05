use std::net::UdpSocket;

use clap::Parser;
use deku::prelude::*;

use mud::args::MudOpts;
use mud::packet::{
	DnsPacket,
	header::DnsHeader,
	question::{DnsQuestion, encode_domain}
};

fn main() {

	let args = MudOpts::parse();

	let packet = DnsPacket {
		header: DnsHeader {
			id: 49_618,
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
			qname: encode_domain(&args.name),
			qtype: 0x0001,
			qclass: 0x0001,
		}
	};

	// UDP
	const BUF_MAX: usize = 4096;
	let mut req  = [0; BUF_MAX];
	let mut res  = [0; BUF_MAX];

	let socket = UdpSocket::bind("0.0.0.0:1053")
		.expect("Could not bind to address");

	let req_vec = packet
		.to_bytes()
		.unwrap();

	let req_len = req_vec.len();

	if req_len < BUF_MAX {

		for i in 0..req_len {
			req[i] = req_vec[i];
		}

		//println!("Sending buffer...");
		socket.send_to(&req[0..req_len], "8.8.8.8:53")
			.expect("Could not send data");

		//println!("Waiting for response...");
		let (number_of_bytes, _src_addr) = socket.recv_from(&mut res)
			.expect("Didn't receive data");

		let modified_message = String::
			from_utf8_lossy(&mut res[..number_of_bytes]);
		//println!("{}", modified_message);
	}
}
