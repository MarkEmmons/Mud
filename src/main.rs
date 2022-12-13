use clap::Parser;
use tracing::{event, span, Level};
use tracing_subscriber;

use mud::client;
use mud::opts::MudOpts;
use mud::packet::DnsPacket;

#[tokio::main]
async fn main() {

	tracing_subscriber::fmt::init();

	let span = span!(Level::TRACE, "Mud");
	let _enter = span.enter();

	event!(Level::INFO, "Parsing user arguments.");
	let opts = MudOpts::parse();

	event!(Level::INFO, "Creating a question packet.");
	let packet = DnsPacket::new_question(&opts);

	event!(Level::INFO, "Sending question packet.");
	let response = client::send_query(&opts, packet)
		.await
		.expect("Failed to receive response");

	event!(Level::INFO, "Printing response info.");
	response.print_response(opts.message_format);
}
