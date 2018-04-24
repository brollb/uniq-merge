// Given a series of input files, merge the uniq -c values

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::collections::HashMap;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage: {} <filenames..>", args[0]);
        std::process::exit(1);
    }

    // Read in the filenames
    let filenames = &args[1..];
    let mut uniq_counts = HashMap::new();

    for filename in filenames {
        let file = File::open(filename).unwrap();
        let file = BufReader::new(file);

        for line in file.lines() {
            let mut chunks = line.as_ref().unwrap().split_whitespace();

            let count_str = chunks.next().unwrap();
            let mut count = count_str.parse::<i32>().unwrap();
            let value = chunks.next().unwrap();

            // Split this into a count and symbol
            if uniq_counts.contains_key(value) {
                count += uniq_counts.get(value).unwrap();
            }
            uniq_counts.insert(value.to_owned(), count.clone());
        }
    }
}
