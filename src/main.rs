use clap::Parser;
use tracing::{span, Level};
use tracing_subscriber;

use mud::client;
use mud::opts::MudOpts;
use mud::packet::DnsPacket;

fn main() {

	tracing_subscriber::fmt::init();

	let span = span!(Level::TRACE, "Mud");
	let _enter = span.enter();

	let opts = MudOpts::parse();

	let packet = DnsPacket::new_question(&opts);

	let response = client::send_query(&opts, packet)
		.expect("Failed to receive response");

	response.print_response();
}
