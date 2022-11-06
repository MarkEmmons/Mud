pub mod udp_client;

use crate::opts::MudOpts;
use crate::packet::DnsPacket;

pub fn send_query(opts: &MudOpts, packet: DnsPacket) -> Result<DnsPacket, std::fmt::Error> {

	//match opts.protocol {
	//	HTTP => todo!(),
	//	TLS => todo!(),
	//	TCP => todo!(),
	//	_ => {
			udp_client::send_query(&opts, packet)
	//	}
	//}
}
