// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[path = "../util.rs"]
mod util;
// END UTIL

use std::{
    collections::HashMap,
    io::{stdout, BufWriter, Write},
};
use util::*;

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let lines = input
        .many_lines(EoF)
        .map(|line_str: String| {
            let pairs_parsed = line_str
                .split(" -> ")
                .map(|p| {
                    p.split(",")
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<_>>();
            (
                (pairs_parsed[0][0], pairs_parsed[0][1]),
                (pairs_parsed[1][0], pairs_parsed[1][1]),
            )
        })
        .collect::<Vec<_>>();

    let mut counts = HashMap::new();

    for ((x1, y1), (x2, y2)) in lines {
        if x1 == x2 {
            let smaller = y1.min(y2);
            let bigger = y1.max(y2);
            for y in smaller..(bigger + 1) {
                *counts.entry((x1, y)).or_insert(0) += 1;
            }
        }

        if y1 == y2 {
            let smaller = x1.min(x2);
            let bigger = x1.max(x2);
            for x in smaller..(bigger + 1) {
                *counts.entry((x, y1)).or_insert(0) += 1;
            }
        }
    }

    let overlap_count = counts.iter().filter(|(_, v)| **v > 1).count();

    writeln!(out, "{}", overlap_count).unwrap();
}
