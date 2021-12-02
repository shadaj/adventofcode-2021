// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[path = "../util.rs"]
mod util;
// END UTIL

use std::io::{stdout, BufWriter, Write};
use util::*;

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut prev: Option<u32> = None;
    let mut count = 0;

    for next in input.many() {
        if let Some(prev) = prev {
            if next > prev {
                count += 1;
            }
        }

        prev = Some(next);
    }

    writeln!(out, "{}", count).unwrap();
}
