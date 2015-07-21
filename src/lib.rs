/*
OscMessage usage:

let mut msg = OscMessage::new();
msg.address("/foo");
msg.add_arg();
msg.add_arg();
msg.args[0].string("some text");
msg.args(1).float32(3.14);
msg.update_typelist();
let payload = msg.get_bytes();
*/

use std::mem;

enum ArgType {
	STRING,
	INT32,
	FLOAT32,
	BLOB
}

pub struct Argument {
	arg_type: ArgType,
	bytearray: Vec<u8>,
}

impl Argument {
	fn new() -> Argument {
		Argument { arg_type: ArgType::STRING, bytearray: Vec::with_capacity(4) }
	}

	pub fn int32(&mut self, i: i32) {
		self.bytearray.clear();

		let raw_bytes: [u8; 4] = unsafe { mem::transmute(i.to_be()) };

		for byte in raw_bytes.iter() {
			self.bytearray.push(*byte);
		}

		self.arg_type = ArgType::INT32;
	}

	pub fn float32(&mut self, f: f32) {
		self.bytearray.clear();

		let raw_bytes: [u8; 4] = unsafe { mem::transmute(f) };

		for byte in raw_bytes.iter().rev() {
			self.bytearray.push(*byte);
		}

		self.arg_type = ArgType::FLOAT32;
	}

	pub fn string(&mut self, s: &str) {
		self.bytearray.clear();

		for byte in s.bytes() {
			self.bytearray.push(byte);
		}
		self.bytearray.push(0);

		while self.bytearray.len() % 4 != 0 {
			self.bytearray.push(0);
		}

		self.arg_type = ArgType::STRING;
	}
}

pub struct OscMessage {
	address: Vec<u8>,
	typelist: Vec<u8>,
	pub args: Vec<Argument>,
	bytes: Vec<u8>
}

impl OscMessage {
	pub fn new() -> OscMessage {
		OscMessage { 
			address: Vec::with_capacity(64), 
			typelist: Vec::with_capacity(8), 
			args: Vec::new(),
			bytes: Vec::with_capacity(256),
		}
	}

	pub fn address(&mut self, address: &str) {
		self.address.clear();

		for byte in address.bytes() {
			self.address.push(byte);
		}
		self.address.push(0);

		while self.address.len() % 4 != 0 {
			self.address.push(0);
		}
	}

	pub fn add_arg(&mut self) {
		self.args.push(Argument::new());
	}

	pub fn update_typelist(&mut self) {
		self.typelist.clear();
		self.typelist.push(44);

		for arg in self.args.iter() {
			match arg.arg_type {
				ArgType::FLOAT32 => self.typelist.push(102),
				ArgType::INT32 => self.typelist.push(105),
				ArgType::STRING => self.typelist.push(115),
				ArgType::BLOB => panic!("blob not supported"),
			}
		}

		while self.typelist.len() % 4 != 0 {
			self.typelist.push(0);
		}
	}

	pub fn get_bytes(&mut self) -> &[u8] {
		self.bytes.clear();
		for byte in self.address.iter() { self.bytes.push(*byte); }
		for byte in self.typelist.iter() { self.bytes.push(*byte); }
		for arg in self.args.iter() {
			for byte in arg.bytearray.iter() {
				self.bytes.push(*byte);
			}
		}
		&self.bytes[..]
	}
}