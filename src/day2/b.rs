// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[path = "../util.rs"]
mod util;
// END UTIL

use std::io::{stdout, BufWriter, Write};
use util::*;

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    loop {
        if let Some(next_command) = input.next::<String>() {
            let next_change: i32 = input.next().unwrap();

            match next_command.as_str() {
                "forward" => {
                    horizontal += next_change;
                    depth += aim * next_change;
                }
                "down" => aim += next_change,
                "up" => aim -= next_change,
                _ => panic!(),
            }
        } else {
            break;
        }
    }

    writeln!(out, "{}", horizontal * depth).unwrap();
}
