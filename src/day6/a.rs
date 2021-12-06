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

    let fish_timers = input.many_by(",", EoF).collect::<Vec<u32>>();
    let mut fish_map = HashMap::new();
    for fish in fish_timers {
        *fish_map.entry(fish).or_insert(0) += 1;
    }

    for _ in 0..80 {
        let new_fish_count = *fish_map.get(&0).unwrap_or(&0);

        let mut new_map = HashMap::new();
        for (&day, &count) in fish_map.iter() {
            if day == 0 {
                *new_map.entry(6).or_insert(0) += count;
            } else {
                *new_map.entry(day - 1).or_insert(0) += count;
            }
        }
        fish_map = new_map;

        *fish_map.entry(8).or_insert(0) += new_fish_count;
    }

    writeln!(out, "{}", fish_map.values().sum::<u64>()).unwrap();
}
