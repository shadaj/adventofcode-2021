// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[path = "../util.rs"]
mod util;
// END UTIL

use std::collections::VecDeque;
use std::io::{stdout, BufWriter, Write};
use util::*;

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut window: VecDeque<i32> = VecDeque::new();
    let mut cur_sum = 0;
    let mut count = 0;

    for next in input.many(EoF) {
        if window.len() == 3 {
            let front_elem = window.pop_front().unwrap();
            let next_window_sum = cur_sum - front_elem + next;
            if next_window_sum > cur_sum {
                count += 1;
            }

            cur_sum -= front_elem;
        }

        window.push_back(next);
        cur_sum += next;
    }

    writeln!(out, "{}", count).unwrap();
}
