use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn exec() {
	// open the input file
	let mut buf = String::new();
	let mut input = File::open("src/day6/input.txt")
	                     .expect("could not read input file");

	input.read_to_string(&mut buf)
	     .expect("could not read file into buffer");

    let num_columns = buf.lines()
                         .peekable()
                         .next().unwrap().len();


    let mut hgs = vec![HashMap::new(); num_columns];
    let hgs_len = hgs.len(); // HACK: avoid borrowing in loop body
    println!("hgs_len => {}", hgs_len);
    for line in buf.lines() {
        let mut col = 0;
        for ch in line.chars() {
            let freq = hgs[col].entry(ch).or_insert(0);
            *freq += 1;
            col = (col + 1) % hgs_len;
        }   
    }

    // sort buckets by freq
    let mut output = vec![];
    for hg in &mut hgs {
        let mut hg = hg.into_iter()
                       .collect::<Vec<_>>();

        hg.sort_by(|a,b| a.1.cmp(&b.1));
        println!("{:?}", hg);
        output.push(*hg[0].0);
    }

    println!("{:?}", output);
}
