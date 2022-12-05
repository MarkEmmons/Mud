use clap::Parser;

/// mud - DNS lookup utility
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct MudOpts {

	/// The name of IP address of the name server to query.
	#[arg(short, long)]
	pub server: String,

	/// The domain name to query.
	#[arg(short = 'q', long)]
	pub name: String,

	/// The resource record type to query.
	#[arg(short = 't', long = "type")]
	pub query_type: String,

	/// The resource record type to query.
	#[arg(short, long, default_value = "udp")]
	pub protocol: String,
}
