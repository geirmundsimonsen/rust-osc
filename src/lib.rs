/*
PURPOSE:

A simple Osc message implementation for rust. No osc bundles
yet. I try to aim for speed of execution, meaning the 
backing byte-array will be assembled after each
add/change/remove opeation. Changes that doesn't alter the
byte structure (like changing a float value) should be speedy.
Changes to a longer/shorter address will be more expensive.

USAGE:

let mut msg = osc2::OscMessage2::new("/foo");
msg.add(Arg::String("bar"));
msg.add(Arg::F32(3.14));
msg.add(Arg::I32(42));

msg.get_bytes() returns a slice of the underlying u8-vector,
send it with UDP.

msg.get_bytes_copy() returns a copy of the vector.

TODO:

implement the change and remove functions.
*/

use std::mem;

pub enum Arg<'a> {
	String(&'a str),
	I32(i32),
	F32(f32),
}

pub struct OscMessage {
	bytes: Vec<u8>,
	typelist: Vec<char>,
	typelist_start: u16,
}

impl OscMessage {
	pub fn new(address: &str) -> OscMessage2 {
		let mut bytes: Vec<u8> = Vec::with_capacity(64);
		let typelist: Vec<char> = Vec::with_capacity(8);

		// address to bytes
		for byte in address.bytes() {
			bytes.push(byte);
		}
		bytes.push(0);

		while bytes.len() % 4 != 0 {
			bytes.push(0);
		}

		let typelist_start = bytes.len() as u16;

		// initialized typelist to bytes
		bytes.push(44);
		bytes.push(0);
		bytes.push(0);
		bytes.push(0);

		// return an initial oscmessage
		OscMessage2 { 
			bytes: bytes, 
			typelist: typelist,
			typelist_start: typelist_start,
		}
	}

	pub fn add(&mut self, arg: Arg) {
		match arg {
			Arg::String(s) => {
				self.add_type('s');

				for byte in s.bytes() {
					self.bytes.push(byte);
				}
				self.bytes.push(0);

				while self.bytes.len() % 4 != 0 {
					self.bytes.push(0);
				}
			},
			Arg::I32(i) => {
				self.add_type('i');

				let raw_bytes: [u8; 4] = unsafe { mem::transmute(i.to_be()) };

				for byte in raw_bytes.iter() {
					self.bytes.push(*byte);
				}
			},
			Arg::F32(f) => {
				self.add_type('f');

				let raw_bytes: [u8; 4] = unsafe { mem::transmute(f) };

				for byte in raw_bytes.iter().rev() {
					self.bytes.push(*byte);
				}
			}
		}
	}

	pub fn change(argNo: u32) {

	}

	pub fn remove(argNo: u32) {

	}

	pub fn removeAll() {

	}

	pub fn address(address: &str) {

	}

	pub fn get_bytes(&self) -> &[u8] {
		&self.bytes[..]
	}

	pub fn get_bytes_copy(&self) -> Vec<u8> {
		self.bytes.clone()
	}

	// method depends on typelist_start and typelist.len()
	// be sure to update typelist_start in other methods
	fn add_type(&mut self, c: char) {
		if (self.typelist.len() + 2) % 4 == 0 {
			// we expand!
			let insert_point = 
				self.typelist_start as usize + (self.typelist.len() / 4 + 1) * 4;
			for _ in 0..4 {
				self.bytes.insert(insert_point, 0);
			}
		}

		self.bytes[self.typelist_start as usize + self.typelist.len() + 1] = c as u8;
		self.typelist.push(c);
	}

	pub fn debug(&self) {
		let mut count = 0;

		for byte in self.bytes.iter() {
			if count % 4 == 0 { println!("") }

			if *byte == 0 {
				print!(".  ");
			} else {
				print!("{} ", *byte);
			}

			count += 1;
		}
	}
}