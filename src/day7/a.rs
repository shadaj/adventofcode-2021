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
    let crab_count = crab_locations.len();
    dbg!(&crab_locations);

    let mut cur_fuel: usize = crab_locations.iter().sum();
    let mut min_fuel = cur_fuel;
    let mut left_of_target: usize = 0;
    let mut cur_target: usize = 0;

    for crab in crab_locations {
        if crab == cur_target {
            left_of_target += 1;
        } else {
            // crab > cur_target
            cur_fuel += (crab - cur_target) * left_of_target;
            cur_fuel -= (crab - cur_target) * (crab_count - left_of_target);

            cur_target = crab;
            left_of_target += 1;

            if cur_fuel < min_fuel {
                min_fuel = cur_fuel;
            }
        }
    }

    writeln!(out, "{}", min_fuel).unwrap();
}
