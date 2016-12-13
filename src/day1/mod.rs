use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

/// Grid navigation
#[derive(Debug)]
enum Turn {
	Left(i64),
	Right(i64),
}

/// Cardinal direction & unit distance.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Compass {
	North,
	East,
	South,
	West,
}

pub fn exec() {
	// open the input file
	let mut buf = String::new();
	let mut input = File::open("src/day1/input.txt")
	                     .expect("could not read input file");

	input.read_to_string(&mut buf)
	     .expect("could not read file into buffer");


	let directions = parse_directions(&mut buf[..]);
	println!("ended up at {:?}", directions);
}

fn parse_directions(buf: &mut str) -> (i64, i64) {
	// split text into a series of turns
	let stream: Vec<_> = buf.split(", ")
	                .map(|buf| buf.trim())
	                .map(|buf| buf.split_at(1))
	                .map(|buf| match buf {
	                	("L", steps) => Turn::Left( steps.parse().unwrap()),
	                	("R", steps) => Turn::Right(steps.parse().unwrap()),
	                	_ => panic!("invalid gps direction (._.*)")
	                })
	                .collect();



	// start at origin facing north
	let mut dir = Compass::North;
	let (mut x, mut y) = (0, 0);

	// memorize previous points on the grid we've visited
	let mut locations = HashSet::new();

	// follow directions
	for entry in stream {
		// update the compass
		let (new_dir,steps) = match (&dir, entry) {
			(&Compass::North, Turn::Right(steps)) => (Compass::East,  steps),
			(&Compass::North, Turn::Left(steps))  => (Compass::West,  steps),
			(&Compass::East,  Turn::Right(steps)) => (Compass::South, steps),
			(&Compass::East,  Turn::Left(steps))  => (Compass::North, steps),
			(&Compass::South, Turn::Right(steps)) => (Compass::West,  steps),
			(&Compass::South, Turn::Left(steps))  => (Compass::East,  steps),
			(&Compass::West,  Turn::Right(steps)) => (Compass::North, steps),
			(&Compass::West,  Turn::Left(steps))  => (Compass::South, steps),
		};

		dir = new_dir;

		match &dir {
			&Compass::North => {
				for i in 0..steps { log(&mut locations, (x, y + i)); }
				y += steps;
			},

			&Compass::South => {
				for i in 0..steps { log(&mut locations, (x, y - i)); }
				y -= steps;
			},

			&Compass::East  => { 
				for i in 0..steps { log(&mut locations, (x + i, y)); }
				x += steps;
			},

			&Compass::West  => { 
				for i in 0..steps { log(&mut locations, (x - i, y)); }
				x -= steps 
			},
		};

		println!("facing: {:?}, x: {}, y: {}", dir, x, y);
	}

	(x, y)
}

fn log(locations: &mut HashSet<(i64, i64)>, dest: (i64, i64)) {
	// HACK: comment out the panic to run to completion
	if locations.contains(&dest) { panic!("been here before ... {:?}", &dest); }
	locations.insert(dest);
}
