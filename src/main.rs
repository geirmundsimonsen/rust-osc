extern crate osc2;

use std::net;
use osc2::Arg;

fn main() {
	let socket = net::UdpSocket::bind("127.0.0.1:12423").unwrap();

	let mut msg = osc2::OscMessage2::new("/foo");
	msg.add(Arg::String("yeys"));
	msg.add(Arg::F32(984.423));
	msg.debug();
	
	socket.send_to(msg.get_bytes(), "127.0.0.1:57120");

	drop(socket);
}
