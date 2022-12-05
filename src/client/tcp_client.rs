use deku::prelude::*;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tracing::{event, Level};

use crate::opts::MudOpts;
use crate::packet::DnsPacket;

pub async fn send_query(_opts: &MudOpts, packet: DnsPacket) -> std::io::Result<DnsPacket> {

	const BUF_MAX: usize = 4096;
	let mut res  = [0; BUF_MAX];

	let mut stream = TcpStream::connect("8.8.8.8:53")
		.await
		.expect("Failed to connect to stream");

	let req = packet
		.to_bytes()
		.unwrap();

	//let length: u16 = req.len() as u16;

	stream.write_all(req.as_ref())
		.await
		.expect("Could not send data");

	let number_of_bytes = stream.read(&mut res)
		.await
		.expect("Didn't receive data");

	event!(Level::INFO, "Received {} bytes: {:?}", number_of_bytes, res);

	let ((rest, offset), result) = DnsPacket::from_bytes(
		(&res[..number_of_bytes], 0)
	).unwrap();

	assert_eq!(rest.len(), 0);
	assert_eq!(offset, 0);

	Ok(result)
}
