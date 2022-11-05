use std::net::UdpSocket;

use clap::Parser;
use deku::prelude::*;
use tracing::{event, span, Level};
use tracing_subscriber;

use mud::opts::MudOpts;
use mud::packet::DnsPacket;

fn main() {

	tracing_subscriber::fmt::init();

	let span = span!(Level::TRACE, "Mud");
	let _enter = span.enter();

	let opts = MudOpts::parse();

	let packet = DnsPacket::new_question(opts);

	// let response = client::send_query(opts, packet);

	// response.print_response();

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

		event!(Level::INFO, "Sending buffer...");
		socket.send_to(&req[0..req_len], "8.8.8.8:53")
			.expect("Could not send data");

		event!(Level::INFO, "Waiting for response...");
		let (number_of_bytes, _src_addr) = socket.recv_from(&mut res)
			.expect("Didn't receive data");

		let modified_message = String::
			from_utf8_lossy(&mut res[..number_of_bytes]);
		println!("{}", modified_message);
	}
}
