use std::fs::File;
use std::io::prelude::*;

///  ## AOC Day 2 (Part 1)
///  
///  You are given a series of instructions (U/R/L/D) which move a cursor
///  around a standard telephone keypad (sans 0-row)
///
///	 Each line is a sequence of steps on a sort of "keypad state machine"
///	 that eventually yield the digit. The state machine starts at the center
///	 digit (5) ...
///

struct HiTekPad {
	cursor: i64,
}

/// State machine for the new HITEK PAD 4000(TM)
///
///     1
///   2 3 4
/// 5 6 7 8 9
///   A B C
///     D
///
impl HiTekPad {
	pub fn new() -> Self { HiTekPad { cursor: 5 } }

	pub fn cursor(&self) -> i64 { self.cursor }

	pub fn up(&mut self) {
		self.cursor = match self.cursor {
			1 | 2 | 4 | 5 | 9 => self.cursor,
			
			0x3 => 0x1,
			0x6 => 0x2,
			0x7 => 0x3,
			0x8 => 0x4,
			0xA => 0x6,
			0xB => 0x7,
			0xC => 0x8,
			0xD => 0xB,

			_ => panic!("invalid keypad state!"),
		};
	}
	
	pub fn down(&mut self) {
		self.cursor = match self.cursor {
			5 | 9 | 0xA | 0xC | 0xD => self.cursor,

			0x1 => 0x3,
			0x2 => 0x6,
			0x3 => 0x7,
			0x4 => 0x8,
			0x6 => 0xA,
			0x7 => 0xB,
			0x8 => 0xC,
			0xB => 0xD,

			_ => panic!("invalid keypad state!"),
		};
	}

	pub fn left(&mut self) {
		self.cursor = match self.cursor {
			1 | 2 | 5 | 0xA | 0xD => self.cursor,

			0x3 => 0x2,
			0x4 => 0x3,
			0x6 => 0x5,
			0x7 => 0x6,
			0x8 => 0x7,
			0x9 => 0x8,
			0xB => 0xA,
			0xC => 0xB,

			_ => panic!("invalid keypad state!"),
		};
	}

	pub fn right(&mut self) {
		self.cursor = match self.cursor {
			1 | 4 | 9 | 0xC | 0xD => self.cursor,

			0x2 => 0x3,
			0x3 => 0x4,
			0x5 => 0x6,
			0x6 => 0x7,
			0x7 => 0x8,
			0x8 => 0x9,
			0xA => 0xB,
			0xB => 0xC,

			_ => panic!("invalid keypad state!"),
		};
	}
}

struct KeyPad {
	cursor: i64,
}

impl KeyPad {
	pub fn new() -> Self { KeyPad { cursor: 5 } }

	pub fn cursor(&self) -> i64 { self.cursor }

	pub fn up(&mut self) {
		self.cursor = match self.cursor {
			1 | 2 | 3 => self.cursor, /* no op */

			4 => 1,
			5 => 2,
			6 => 3,

			7 => 4,
			8 => 5,
			9 => 6,

			_ => panic!("not a digit on the keypad"),
		};
	}

	pub fn down(&mut self) {
		self.cursor = match self.cursor {
			7 | 8 | 9 => self.cursor,

			1 => 4,
			2 => 5,
			3 => 6,

			4 => 7,
			5 => 8,
			6 => 9,

			_ => panic!("not a digit on the keypad"),
		};
	}

	pub fn left(&mut self) {
		self.cursor = match self.cursor {
			1 | 4 | 7 => self.cursor,

			2 => 1,
			5 => 4,
			8 => 7,

			3 => 2,
			6 => 5,
			9 => 8,

			_ => panic!("not a digit on keypad"),
		};
	}

	pub fn right(&mut self) {
		self.cursor = match self.cursor {
			3 | 6 | 9 => self.cursor,

			1 => 2,
			4 => 5,
			7 => 8,
	
			2 => 3,
			5 => 6,
			8 => 9,

			_ => panic!("not a digit on keypad"),
		};
	}
}

pub fn exec() {
	// open the input file
	let mut buf = String::new();
	let mut input = File::open("src/day2/input.txt")
	                     .expect("could not read input file");

	input.read_to_string(&mut buf)
	     .expect("could not read file into buffer");


	let digits: Vec<_> = buf.lines()
	                        .map(|line| parse_line(&line))
	                        .collect();

	for digit in &digits {
		println!("{:x} is code", digit);
	}
}


fn parse_line(buf: &str) -> i64 {
	let mut keypad = HiTekPad::new();

	for dir in buf.chars() {
		match dir {
			'U' => keypad.up(),
			'D' => keypad.down(),
			'L' => keypad.left(),
			'R' => keypad.right(),

			_ => panic!("unknown direction {}", dir),
		}
	}

	keypad.cursor()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_keypad_fsm() {
		let mut keypad = KeyPad::new();
		keypad.up();    // 2
		keypad.up();    // 2
		keypad.right(); // 3
		keypad.right(); // 3
		keypad.down();  // 6
		keypad.down();  // 9
		keypad.down();  // 9
		keypad.left();  // 8
		keypad.left();  // 7
		keypad.left();  // 7

		assert_eq!(keypad.cursor(), 7);
	}

	#[test]
	fn tset_hitek_fsm() {
		let mut keypad = HiTekPad::new();
		keypad.up();    // 5
		keypad.left();  // 5
		keypad.down();  // 5
		keypad.right(); // 6
		keypad.up();    // 2
		keypad.up();    // 2
		keypad.right(); // 3
		keypad.up();    // 1
		keypad.up();    // 1
		keypad.down();  // 3
		keypad.right(); // 4
		keypad.right(); // 4
		keypad.down();  // 8
		keypad.down();  // C
		keypad.right(); // C
		keypad.left();  // B
		keypad.down();  // D

		assert_eq!(keypad.cursor(), 0xD);
	}
}

