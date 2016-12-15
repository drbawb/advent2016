use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::num::Wrapping;

pub fn exec() {
	// open the input file
	let mut buf = String::new();
	let mut input = File::open("src/day7/input.txt")
	                     .expect("could not read input file");

	input.read_to_string(&mut buf)
	     .expect("could not read file into buffer");

    // parse a query
    //let query: Vec<_> = "ioxxoj[asdfgh]zxcvbn".chars().collect();
    //let (start, end) = lps_manacher(&query);
    //let subseq = &query[start..end].iter()
    //                               .cloned()
    //                               .collect::<String>();


    // let tests = [
    //     (true,  "aba[bab]xyz"),
    //     (false, "xyx[xyx]xyx"),
    //     (true,  "aaa[kek]eke"),
    //     (true,  "zazbz[bzb]cdb"),
    // ];

    // for &(res,test) in &tests {
    //     println!("test({}) => {}, expect: {}", test, supports_ssl(&test), res);
    // }

     let ips: Vec<_> = buf.lines().map(|line| {
         supports_ssl(line)
     }).filter(|&tls| tls).collect();

    println!("ips -> {:?}", ips.len());
}

fn supports_ssl(buf: &str) -> bool {
    let mut hnets = vec![];
    let mut norms = vec![];

    // walk the string storing ranges ...
    let mut l_pos = 0; let mut r_pos = 0;
    let mut is_hnet = false;
    for (chi, ch) in buf.chars().enumerate() {
        match ch {
            'a'...'z' => r_pos = chi,

            '['      => { 
                norms.push((l_pos, r_pos));
                l_pos = chi + 1; r_pos = chi + 1;
            },

            ']'      => { 
                hnets.push((l_pos, r_pos));
                l_pos = chi + 1; r_pos = chi + 1;
            },

            err => panic!("unexpected character in ip: {}", err),
        }
    }

    norms.push((l_pos, r_pos));



    for &(l,r) in &norms {
        let abas = is_aba(&buf[l..r+1].as_bytes());
        for aba in abas {
            for &(hl, hr) in &hnets {
                if has_bab(&buf[hl..hr+1].as_bytes(), aba) { return true; }
            }
        }
    }

    return false;
}

fn supports_tls(buf: &str) -> bool {
    let mut hnets = vec![];
    let mut norms = vec![];

    // walk the string storing ranges ...
    let mut l_pos = 0; let mut r_pos = 0;
    let mut is_hnet = false;
    for (chi, ch) in buf.chars().enumerate() {
        match ch {
            'a'...'z' => r_pos = chi,

            '['      => { 
                norms.push((l_pos, r_pos));
                l_pos = chi + 1; r_pos = chi + 1;
            },

            ']'      => { 
                hnets.push((l_pos, r_pos));
                l_pos = chi + 1; r_pos = chi + 1;
            },

            err => panic!("unexpected character in ip: {}", err),
        }
    }

    norms.push((l_pos, r_pos));

    let abba_norms = norms.iter().map(|&(l,r)| {
        is_abba(&buf[l..r+1].as_bytes())
    }).fold(false, |acc,el| acc || el);

    let abba_hnets = hnets.iter().map(|&(l,r)| {
        is_abba(&buf[l..r+1].as_bytes())
    }).fold(false, |acc,el| acc || el);

    abba_norms  && !abba_hnets
}

fn is_aba(buf: &[u8]) -> Vec<&[u8]> {
    let mut pos = 0;
    let mut out = vec![];
    while (pos + 2) < buf.len() { // stop when we walk off the end
        let a = buf[pos + 0];
        let b = buf[pos + 1];
        let c = buf[pos + 2];
        if (a == c) && (a != b) { out.push(&buf[pos..pos+3]) }
        pos += 1;
    }

    out
}

fn has_bab(buf: &[u8], pattern: &[u8]) -> bool {
    let pa = pattern[0];
    let pb = pattern[1];
    let pc = pattern[2];

    let mut pos = 0;
    while (pos + 2) < buf.len() {
        let ba = buf[pos + 0];
        let bb = buf[pos + 1];
        let bc = buf[pos + 2];
       
        if (pa == bb) && (pc == bb) && (ba == pb) && (bc == pb) { return true; }
        pos += 1;
    }

    return false;
}

fn is_abba(buf: &[u8]) -> bool {
    let mut pos = 0;

    while (pos + 3) < buf.len() { // stop when we walk off the end
        let a = buf[pos + 0];
        let b = buf[pos + 1];
        let c = buf[pos + 2];
        let d = buf[pos + 3];
        
        let palindromic = (a == d) && (b == c);
        let difference  = (a != b) && (b != d);

        if palindromic && difference { return true; }
        pos += 1;
    }

    return false;
}


// Manacher's algorithm
// direct translation of code from
// http://algs4.cs.princeton.edu/53substring/Manacher.java.html
// (only differences: using an enum & parametric polymorphism)

#[derive(PartialEq, Eq)]
enum Elt<T> {
    Start, End, Mid,
    Symb(T),
}

fn lps_manacher<T:Clone+Eq>(arr: &[T]) -> (usize, usize) {
    fn preprocess<T:Clone>(arr: &[T]) -> Vec<Elt<T>> {
        let n = arr.len();
        let mut vec = Vec::with_capacity(2*n+5);
        vec.push(Elt::Start);
        vec.push(Elt::Mid);
        for i in 0..n {
            vec.push(Elt::Symb(arr[i].clone()));
            vec.push(Elt::Mid);
        }
        vec.push(Elt::End);
        vec
    }

    let vec = preprocess(arr);
    let n = vec.len();

    let mut lps = vec![0; n];
    let mut center = 0;
    let mut right = 0;

    for i in 1..(n-1) {
        if (2 * center) > i { 
            let mirror = (2 * center) - i;
            if right > i {
                lps[i] = cmp::min(right - i, lps[mirror]);
            }
        }

        while vec[i + 1 + lps[i]] == vec[i - 1 - lps[i]] {
            lps[i] += 1;
        }

        if i + lps[i] > right {
            center = i;
            right = i + lps[i];
        }
    }

    // Not idiomatic Rust!
    let mut max_len = 0;
    let mut max_center = 0;
    for i in 1..(n-1) {
        if lps[i] > max_len {
            max_len = lps[i];
            max_center = i;
        }
    }
    
    ((max_center - 1 - max_len) / 2, (max_center - 1 + max_len) / 2)
}
