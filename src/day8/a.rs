// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[path = "../util.rs"]
mod util;
// END UTIL

use std::{
    collections::HashSet,
    io::{stdout, BufWriter, Write},
};
use util::*;

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut entries: Vec<(Vec<String>, Vec<String>)> = vec![];
    while let Ok(entry) = input.next_line::<String>() {
        let (before, after) = entry.split_once(" | ").unwrap();
        entries.push((
            before.split_whitespace().map(|s| s.to_string()).collect(),
            after.split_whitespace().map(|s| s.to_string()).collect(),
        ));
    }

    let lengths = HashSet::from([2, 4, 3, 7]);

    let mut count = 0;
    for (_, after) in entries {
        for v in after {
            if lengths.contains(&v.len()) {
                count += 1;
            }
        }
    }

    writeln!(out, "{}", count).unwrap();
}
