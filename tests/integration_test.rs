use mud_lib::{
	client,
	opts::MudOpts,
	packet::DnsPacket
};

#[tokio::test]
async fn it_gets_an_ip() {

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

	let response = client::send_query(&opts, packet)
		.await
		.expect("Failed to receive response");

	assert_eq!(response.header.an_count, 1);
	assert_eq!(response.answers[0].rdlength, 4);
}
