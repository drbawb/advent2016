use std::fs::File;
use std::io::prelude::*;

struct Display {
	pxbuf: Vec<Vec<bool>>, // pxbuf[row][col]
}

impl Display {
	pub fn new() -> Display {
		Display {
			pxbuf: vec![vec![false; 50]; 6],
		}
	}

	pub fn draw_rect(&mut self, w: usize, h: usize) {
		for row in 0..h {
			for col in 0..w {
				self.pxbuf[row][col] = true;
			}
		}
	}

	pub fn count_pixels(&self) -> usize {
		let mut counter = 0;
		for row in &self.pxbuf {
			for col in row { 
				if *col { counter += 1 }
			}
		}

		counter
	}

	pub fn shift_row(&mut self, y: usize, width: usize) {
		for _step in 0..width {
			let last = self.pxbuf[y].pop().unwrap();
			self.pxbuf[y].insert(0, last);
		}
	}

	pub fn shift_col(&mut self, x: usize, width: usize) {
		// read col x into working buffer
		let mut buf = vec![];
		for row in 0..6 {
			buf.push(self.pxbuf[row][x]);
		}
		assert!(buf.len() == 6);

		for _step in 0..width {
			let last = buf.pop().unwrap();
			buf.insert(0, last);
		}

		// put buf back into columns
		for row in 0..6 {
			self.pxbuf[row][x] = buf[row];
		}
	}

	pub fn print(&self) {
		for row in &self.pxbuf {
	 		for col in row {
				print!("{}", if *col { "@" } else { "." });
			}
			print!("\n");
		}
	}
}

pub fn exec() {
	// open the input file
	let mut buf = String::new();
	let mut input = File::open("src/day8/input.txt")
	                     .expect("could not read input file");

	input.read_to_string(&mut buf)
	     .expect("could not read file into buffer");

	let mut display = Display::new();

	for line in buf.lines() {
		if line.starts_with("rect") {
			let spec = &line[5..].split("x")
			                     .map(|num| num.parse().unwrap())
			                     .collect::<Vec<_>>();

			display.draw_rect(spec[0], spec[1]);
		} else if line.starts_with("rotate row") {
			let (y, width) = parse_rotate_row(line);
			display.shift_row(y, width);
		} else if line.starts_with("rotate column") {
			let (x, width) = parse_rotate_col(line);
			display.shift_col(x, width);
		} else { panic!("unhandled instruction: {}", line); }
	}

	println!("display pixels lit: {}", display.count_pixels());
	display.print();
}

fn parse_rotate_row(buf: &str) -> (usize, usize) {
	// rotate row y=0 by 20
	// find indices of interest
	let col   = buf.find("y=").unwrap();
	let shift = buf.find("by").unwrap();

	// parse out relevant parts
	let y = buf[(col+2)..(shift-1)].parse().unwrap(); // [y=]..[by]
	let width = buf[(shift+3)..].parse().unwrap(); // [by ]..$end

	(y, width)
}

fn parse_rotate_col(buf: &str) -> (usize, usize) {
	// rotate column x=0 by 20
	// find indices of interest
	let col   = buf.find("x=").unwrap();
	let shift = buf.find("by").unwrap();

	// parse out relevant parts
	let x = buf[(col+2)..(shift-1)].parse().unwrap(); // [y=]..[by]
	let width = buf[(shift+3)..].parse().unwrap(); // [by ]..$end

	(x, width)
}
