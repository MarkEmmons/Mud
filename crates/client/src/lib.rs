pub mod config;
pub mod tcp_client;
pub mod udp_client;

pub extern crate mud_opts as opts;
pub extern crate mud_packet as packet;

use opts::MudOpts;
use packet::DnsPacket;

// TODO: Resolve nameserver from Windows registry

pub async fn send_query(opts: &MudOpts, packet: DnsPacket) -> std::io::Result<DnsPacket> {

	let cfg = config::ClientConfig::new().await?;

	match opts.protocol.to_lowercase().as_str() {
		"http" => todo!(),
		"tls" => todo!(),
		"tcp" => tcp_client::send_query(&opts, packet).await,
		_ => udp_client::send_query(&opts, packet, cfg).await,
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[tokio::test]
	async fn it_reads_etc_resolve() {

		let opts = MudOpts {

			server: String::from("127.0.0.1"),
			port: 53,
			name: String::from("www.archlinux.org"),
			query_type: String::from("ANY"),
			protocol: String::from("udp"),
			message_format: String::from("dig"),
			listen: false,
		};

		let packet = DnsPacket::new_question(&opts);

		send_query(&opts, packet)
			.await
			.expect("Failed to test");
	}
}
