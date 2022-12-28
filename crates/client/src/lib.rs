pub mod config;
pub mod tcp_client;
pub mod udp_client;

pub extern crate mud_opts as opts;
pub extern crate mud_packet as packet;

use opts::MudOpts;
use packet::DnsPacket;

// TODO: Resolve nameserver from Windows registry

// TODO: Move this!
use tokio::net::UdpSocket;
use deku::prelude::*;
//

pub async fn listen(opts: &MudOpts) -> std::io::Result<DnsPacket> {

	let socket = UdpSocket::bind("0.0.0.0:5353").await.unwrap();

	loop {

		// Listen on a host/port
		let mut res = [0; 1024];
		let (number_of_bytes, _addr) = socket.recv_from(&mut res).await.unwrap();

		// When we get a message, try to serialize it to a DnsPacket
		//event!(Level::INFO, "Intercepted a packet.");
		let ((_rest, _offset), packet) = DnsPacket::from_bytes(
			(&res[..number_of_bytes], 0)
		).unwrap();

		// Send the DnsPacket like normal and print
		//event!(Level::INFO, "Sending the packet.");
		let response = send_query(&opts, packet)
			.await
			.expect("Failed to receive response");

		//event!(Level::INFO, "Printing response info.");
		response.print_response(opts.message_format.clone());
	}
}

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
