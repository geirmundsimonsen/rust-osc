extern crate osc2;

use std::net;

fn main() {
	let socket = net::UdpSocket::bind("127.0.0.1:12423").unwrap();

	let mut msg = osc2::OscMessage::new();
	msg.address("/foo");
	msg.add_arg();
	msg.add_arg();
	msg.args[0].string("yeys");
	msg.args[1].float32(984.423);
	msg.update_typelist();
	
	for _ in 0..100 { 
		socket.send_to(msg.get_bytes(), "127.0.0.1:57120");
		std::thread::sleep_ms(1);
	}
	drop(socket);
}
