use std::net::UdpSocket;

use deku::prelude::*;
use tracing::{event, Level};

use crate::opts::MudOpts;
use crate::packet::DnsPacket;

pub fn send_query(_opts: &MudOpts, packet: DnsPacket) -> Result<DnsPacket, std::fmt::Error> {

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
		//socket.send_to(req_vec.as_ref(), "8.8.8.8:53")
		//	.expect("Could not send data");
		socket.send_to(&req[0..req_len], "8.8.8.8:53")
			.expect("Could not send data");

		event!(Level::INFO, "Waiting for response...");
		let (number_of_bytes, _src_addr) = socket.recv_from(&mut res)
			.expect("Didn't receive data");

		//let (_rest, result) = DnsPacket::from_bytes(
		//	(&res[..number_of_bytes], number_of_bytes)
		//).unwrap();
		let modified_message = String::
			from_utf8_lossy(&mut res[..number_of_bytes]);
		println!("{}", modified_message);

		Ok(packet)

	} else {

		Err(std::fmt::Error)
	}
}
