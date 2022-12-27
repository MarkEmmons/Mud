use clap::Parser;

/// mud - DNS lookup utility
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct MudOpts {

	/// The name of IP address of the name server to query.
	#[arg(short, long)]
	pub server: String,

	/// The name of IP address of the name server to query.
	#[arg(short, long, default_value_t = 53)]
	pub port: u16,

	/// The domain name to query.
	#[arg(short = 'q', long)]
	pub name: String,

	/// The resource record type to query.
	#[arg(short = 't', long = "type")]
	pub query_type: String,

	/// The transport layer protocol to use.
	#[arg(short = 'P', long, default_value = "udp")]
	pub protocol: String,

	/// The output format.
	#[arg(short, long, default_value = "dig")]
	pub message_format: String,

	/// The resource record type to query.
	#[arg(short, long)]
	pub listen: bool,
}
