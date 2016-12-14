use std::fs::File;
use std::io::prelude::*;

pub fn exec() {
	// open the input file
	let mut buf = String::new();
	let mut input = File::open("src/day3/input.txt")
	                     .expect("could not read input file");

	input.read_to_string(&mut buf)
	     .expect("could not read file into buffer");


	// solution to part 1
	// ------------------
	// let tris: Vec<_> = buf.lines()
	//                       .map(|line| parse_line(&line))
	//                       .collect();

	let tris = parse_buf(&buf);

	let mut num_valid = 0;
	let mut num_total = 0;
	for tri in &tris {
		num_total += 1;

		let test_1 = (tri[0] + tri[1]) > tri[2];
		let test_2 = (tri[0] + tri[2]) > tri[1];
		let test_3 = (tri[1] + tri[2]) > tri[0];

		if test_1 && test_2 && test_3 { num_valid += 1 }
	}

	println!("{} total, {} valid", num_total, num_valid);
}

// parse buf in column major order instead
fn parse_buf(buf: &str) -> Vec<[i64; 3]> {
	let lulz: Vec<_> = buf.lines()
	                      .map(|line| parse_line(&line))
						  .collect();

	let mut tris = vec![];
	let mut lidx = 0;
	while (lidx + 2) < lulz.len() {
		let tri_1 = [lulz[lidx + 0][0], lulz[lidx + 1][0], lulz[lidx + 2][0]];
		let tri_2 = [lulz[lidx + 0][1], lulz[lidx + 1][1], lulz[lidx + 2][1]];	
		let tri_3 = [lulz[lidx + 0][2], lulz[lidx + 1][2], lulz[lidx + 2][2]];

		tris.extend_from_slice(&[tri_1, tri_2, tri_3]);
		lidx += 3; // skip 3 lines
	}

	tris
}

// input is: "  ddd  ddd  ddd"
// where a digit is either blank and/or a digit-string
fn parse_line(buf: &str) -> [i64; 3] {
	let d1 = buf[02..05].trim().parse().expect("could not parse side length");
	let d2 = buf[07..10].trim().parse().expect("could not parse side length");
	let d3 = buf[12..15].trim().parse().expect("could not parse side length");

	[d1, d2, d3]
}
