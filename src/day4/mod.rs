use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

enum ParseMode {
	PeekNext, /* maybe letter OR number ... */
	Letter,   /* read letters to buf until hyphen */
	Number,   /* read numbers to buf until hyphen */
	Hash,     /* read hash until close bracket */
	Stop,     /* end of string reached sucessfully */
}

struct RoomEntry {
	digits:   i64,
	checksum: String,
	letters:  String,
}

pub fn exec() {
	// open the input file
	let mut buf = String::new();
	let mut input = File::open("src/day4/input.txt")
	                     .expect("could not read input file");

	input.read_to_string(&mut buf)
	     .expect("could not read file into buffer");


	// load the roooooms! DO IT! JUST DO IT!
	let mut rooms: Vec<_> = buf.lines()
	                       .map(|line| parse_line(&line))
	                       .filter(|room| is_valid(&room))
						   .map(|room| decrypt_room(&room))
						   .collect();

	rooms.sort();
	for room in &rooms { println!("{:?}", room); }
}

fn decrypt_room(room: &RoomEntry) -> (String, i64) {
	let shift = (room.digits % 26) as u8;
	println!("shift ... {}", shift);
	let out = room.letters.chars().map(|ch| {
		if ch == ' ' { ch }
		else {
			let alpha = (ch as u8) - 97;      // distance from ascii a
			let rot   = (alpha + shift) % 26; // restrict shift to alpha
			(rot + 97) as char                // back to ascii
		}
	}).collect();

	(out, room.digits)
}

fn is_valid(room: &RoomEntry) -> bool {
	let mut histogram = HashMap::new();
	for c in room.letters.chars() {
		if c == ' ' { continue }
		let counter = histogram.entry(c).or_insert(0);
		*counter += 1;
	}

	// sort histogram by frequency
	let mut histogram: Vec<_> = histogram.into_iter().collect();
	histogram.sort_by(|&(ch_a, cnt_a), &(ch_b, cnt_b)| {
		if cnt_a == cnt_b { ch_a.cmp(&ch_b) } // resolve ties alphabetically
		else { cnt_b.cmp(&cnt_a) }              // else sort by frequency
	});

	// take five highest buckets
	let buckets = histogram.into_iter().take(5);
	let digest: String = buckets.into_iter()
	                            .map(|val| val.0)
	                            .collect();

	digest == room.checksum
}

fn parse_line(buf: &str) -> RoomEntry {
	let mut mode = ParseMode::PeekNext;
	let mut abuf = vec![];
	let mut dbuf = vec![];
	let mut hbuf = vec![];

	let mut tokens = buf.chars().peekable();
	'parser: loop {
		match mode {
			ParseMode::PeekNext => mode = match tokens.peek() {
				Some(token) => mode_from_char(*token),
				None        => panic!("could not peek!"),
			},

			ParseMode::Letter => match tokens.next() {
				Some('-')   => { abuf.push(' '); mode = ParseMode::PeekNext },
				Some(token) => abuf.push(token),
				None        => panic!("ran out of letters"),
			},

			ParseMode::Number => match tokens.next() {
				Some('[')   => mode = ParseMode::Hash,
				Some(token) => dbuf.push(token),
				None        => panic!("ran out of numbers"),
			},

			ParseMode::Hash => match tokens.next() {
				Some(']')   => mode = ParseMode::Stop,
				Some(token) => hbuf.push(token),
				None        => panic!("ran out of hash"),
			},

			ParseMode::Stop => break 'parser,
		}
	}

	RoomEntry {
		letters:  abuf.into_iter().collect(),
		digits:   dbuf.into_iter().collect::<String>().parse().unwrap(),
		checksum: hbuf.into_iter().collect(),
	}
}

fn mode_from_char(c: char) -> ParseMode {
	if c >= '0' && c <= '9' { ParseMode::Number }
	else                    { ParseMode::Letter }
}
