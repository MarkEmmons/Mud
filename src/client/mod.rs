pub mod tcp_client;
pub mod udp_client;

use crate::opts::MudOpts;
use crate::packet::DnsPacket;

pub async fn send_query(opts: &MudOpts, packet: DnsPacket) -> std::io::Result<DnsPacket> {

	match opts.protocol.to_lowercase().as_str() {
		"http" => todo!(),
		"tls" => todo!(),
		"tcp" => tcp_client::send_query(&opts, packet).await,
		_ => udp_client::send_query(&opts, packet).await,
	}
}
