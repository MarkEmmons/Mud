use std::io::{Error, ErrorKind};
use deku::prelude::*;
use tokio::net::UdpSocket;

use crate::client::config;
use crate::opts::MudOpts;
use crate::packet::DnsPacket;

pub async fn send_query(_opts: &MudOpts, packet: DnsPacket, cfg: config::ClientConfig) -> std::io::Result<DnsPacket> {

	const BUF_MAX: usize = 512;
	let mut res  = [0; BUF_MAX];

	let socket = UdpSocket::bind("0.0.0.0:1053")
		.await
		.expect("Could not bind to address");

	let req = packet
		.to_bytes()
		.unwrap();

	if req.len() < BUF_MAX {

		socket.send_to(req.as_ref(), cfg.nameserver + ":53")
			.await
			.expect("Could not send data");

		let (number_of_bytes, _src_addr) = socket.recv_from(&mut res)
			.await
			.expect("Didn't receive data");

		let ((rest, offset), result) = DnsPacket::from_bytes(
			(&res[..number_of_bytes], 0)
		).unwrap();

		assert_eq!(rest.len(), 0);
		assert_eq!(offset, 0);

		Ok(result)

	} else {

		Err(Error::new(ErrorKind::Other, "Invalid request length"))
	}
}
