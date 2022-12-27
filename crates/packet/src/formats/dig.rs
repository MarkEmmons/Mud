use crate::DnsPacket;

pub fn print_dig(packet: &DnsPacket) {

	println!("; <<>> Mud 0.0.1 <<>> TODO");
	println!(";; global options: TODO");

	println!(";; Got answer:");
	packet.header.print();

	for q in packet.questions.iter() { q.print(); }
	for a in packet.answers.iter() { a.print(); }
	for a in packet.authority.iter() { a.print(); }
	for a in packet.additional.iter() { a.print(); }

	println!(";; STATS - TODO");
	println!("");
}
