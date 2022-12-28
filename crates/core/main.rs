use clap::Parser;
use tracing::{event, span, Level};
use tracing_subscriber;

use mud_lib::opts::MudOpts;
use mud_lib::client;
use mud_lib::packet::DnsPacket;

#[tokio::main]
async fn main() {

	tracing_subscriber::fmt::init();

	let span = span!(Level::TRACE, "Mud");
	let _enter = span.enter();

	event!(Level::INFO, "Parsing user arguments.");
	let opts = MudOpts::parse();

	if opts.listen {

		// TODO: Cleanup...
		event!(Level::INFO, "Starting listen...");
		client::listen(&opts)
			.await
			.expect("Failed while listening on 0.0.0.0:5353");

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
