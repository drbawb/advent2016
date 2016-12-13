use crypto::md5::Md5;
use crypto::digest::Digest;

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


	let mut counter = 0;
	
	'inner: for i in 0..::std::u32::MAX {
		// re-initialize MD5 digest
		let round = format!("{}{}", input, i);
		hasher.input(round.as_bytes());
		hasher.result(&mut hash);
		hasher.reset();

		// check first five leading zeroes
		if hash[0] != 0x00 || hash[1] != 0x00 { continue 'inner }
		if hash[2] & 0xF0 != 0 { continue 'inner }

		println!("found: {:?}", hash);
		println!("input: {}", round);
		let pos = (hash[2] & 0x0F) as usize;
		if pos > output.len() { continue 'inner }

		// store character if in bounds ...
		let ch      = hash[3] & 0xF0;
		output[pos] = Some(ch);
		counter += 1; if counter >= 8 { break; }
	}

	for byte in &output {
		println!("output {:x}", byte.unwrap());
	}
}
