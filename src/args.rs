use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct MudOpts {

	/// Name of a person to greet
	#[arg(short, long)]
	pub server: String,

	/// Name of a person to greet
	#[arg(short = 'q', long)]
	pub name: String,

	/// Number of times to greet
	#[arg(short = 't', long = "type")]
	pub query_type: String,
}
