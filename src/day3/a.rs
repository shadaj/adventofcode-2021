// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[path = "../util.rs"]
mod util;
// END UTIL

use std::io::{stdout, BufWriter, Write};
use util::*;

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut counts = Vec::new();
    let mut total_count = 0;

    for next in input.many::<String>() {
        if counts.len() == 0 {
            counts = next.chars().map(|_| 0).collect();
        }

        next.chars().enumerate().for_each(|(i, c)| {
            let is_true = c == '1';
            if is_true {
                counts[i] += 1;
            }
        });

        total_count += 1;
    }

    let mut min_for_max = total_count / 2;
    if total_count % 2 == 1 {
        min_for_max += 1;
    }

    let gamma = isize::from_str_radix(
        &counts
            .iter()
            .map(|count| if *count >= min_for_max { '0' } else { '1' })
            .collect::<String>(),
        2,
    )
    .unwrap();

    let epsilon = isize::from_str_radix(
        &counts
            .iter()
            .map(|count| if *count >= min_for_max { '1' } else { '0' })
            .collect::<String>(),
        2,
    )
    .unwrap();

    writeln!(out, "{}", gamma * epsilon).unwrap();
}
