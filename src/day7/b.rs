// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[path = "../util.rs"]
mod util;
// END UTIL

use std::io::{stdout, BufWriter, Write};
use util::*;

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut crab_locations = input.many_by(",", EoF).collect::<Vec<usize>>();
    crab_locations.sort();

    let mut cur_fuel: usize = usize::MAX;
    let mut min_fuel = cur_fuel;

    for crab in crab_locations[0]..(crab_locations[crab_locations.len() - 1] + 1) {
        cur_fuel = 0;
        for &o_crab in crab_locations.iter() {
            let distance = if crab > o_crab {
                crab - o_crab
            } else {
                o_crab - crab
            };

            cur_fuel += (distance * (distance + 1)) / 2;
        }

        if cur_fuel < min_fuel {
            min_fuel = cur_fuel;
        }
    }

    writeln!(out, "{}", min_fuel).unwrap();
}
