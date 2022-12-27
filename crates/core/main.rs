use clap::Parser;
use tracing::{event, span, Level};
use tracing_subscriber;

use mud_lib::cli::opts::MudOpts;
use mud_lib::client;
use mud_lib::packet::DnsPacket;

// TODO: Move this!
use tokio::net::UdpSocket;
use deku::prelude::*;
//

#[tokio::main]
async fn main() {

	tracing_subscriber::fmt::init();

	let span = span!(Level::TRACE, "Mud");
	let _enter = span.enter();

	event!(Level::INFO, "Parsing user arguments.");
	let opts = MudOpts::parse();

	if opts.listen {

		let socket = UdpSocket::bind("0.0.0.0:5353").await.unwrap();

		loop {

			// Listen on a host/port
			let mut res = [0; 1024];
			let (number_of_bytes, _addr) = socket.recv_from(&mut res).await.unwrap();

			// When we get a message, try to serialize it to a DnsPacket
			event!(Level::INFO, "Intercepted a packet.");
			let ((_rest, _offset), packet) = DnsPacket::from_bytes(
				(&res[..number_of_bytes], 0)
			).unwrap();

			// Send the DnsPacket like normal and print
			event!(Level::INFO, "Sending the packet.");
			let response = client::send_query(&opts, packet)
				.await
				.expect("Failed to receive response");

			event!(Level::INFO, "Printing response info.");
			response.print_response(opts.message_format.clone());
		}

	} else {

		event!(Level::INFO, "Creating a question packet.");
		let packet = DnsPacket::new_question(&opts);

		event!(Level::INFO, "Sending question packet.");
		let response = client::send_query(&opts, packet)
			.await
			.expect("Failed to receive response");

		event!(Level::INFO, "Printing response info.");
		response.print_response(opts.message_format);
	}
}
