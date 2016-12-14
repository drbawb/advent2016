use crypto::md5::Md5;
use crypto::digest::Digest;
use std::io::{self, Write};
use std::time::{Duration, SystemTime};
use rand::{thread_rng, Rng};

pub fn exec() {
	let input = "reyedfim"; // from the website...


	let mut output = vec![None; 8];
	let mut hasher = Md5::new();
	let mut hash = [0; 16];

	// let mut res = [0; 16];
	// let derp = "abc3231929";
	// hasher.input(derp.as_bytes());
	// hasher.result(&mut res);
	// println!("{:?}", res);
	// return;

    let mut last_draw_at = SystemTime::now();
	for i in 0..::std::u32::MAX {
		// re-initialize MD5 digest
		let round = format!("{}{}", input, i);
		hasher.input(round.as_bytes());
		hasher.result(&mut hash);
		hasher.reset();

        let dt = last_draw_at.elapsed().unwrap();
        if dt > Duration::from_millis(60) {
            last_draw_at = SystemTime::now();
            print_output(&output);
        }

		// check first five leading zeroes
		if hash[0] != 0x00 || hash[1] != 0x00 { continue }
		if hash[2] & 0xF0 != 0 { continue }

		// println!("found: {:?}", hash);
		// println!("input: {}", round);
		let pos = (hash[2] & 0x0F) as usize;
		if pos >= output.len() { continue }

		// store character if in bounds ...
		let ch      = hash[3] & 0xF0;
        if output[pos].is_none() { output[pos] = Some(ch); } 

        if finished_with(&output) { break; }
	}

    print_output(&output);
    // println!("output {:?}", output);
	// for byte in &output {
	// 	println!("output {:x}", byte.unwrap());
	// }
}

fn print_output(buf: &[Option<u8>]) {
    for _pos in 0..8 { print!("\x08"); } // clear

    let rand_pool = (0..16).collect::<Vec<u8>>();


    for ch in buf {
        match ch {
            &Some(ch) => print!("{:x}", (ch >> 4)),
            &None => print!("{:x}", rand_pool[thread_rng().gen_range(0, 16) as usize]),
        };
    }

    io::stdout().flush().unwrap();
}

fn finished_with(buf: &[Option<u8>]) -> bool {
    for el in buf.iter().take(8) {
        if el.is_none() { return false; }
    }

    true
}
