// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
mod util {
    use std::{io::stdin, str::FromStr};

    pub struct ScannerManyIterator<'a, T: FromStr> {
        scanner: &'a mut Scanner,
        phantom: std::marker::PhantomData<T>,
    }

    impl<'a, T: FromStr> Iterator for ScannerManyIterator<'a, T> {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            self.scanner.next()
        }
    }

    pub struct Scanner {
        buffer: Vec<String>,
    }

    impl Scanner {
        pub fn new() -> Scanner {
            Scanner { buffer: Vec::new() }
        }

        pub fn next<T: std::str::FromStr>(&mut self) -> Option<T> {
            loop {
                if let Some(token) = self.buffer.pop() {
                    return Some(token.parse().ok().expect("Failed parse"));
                }

                let mut input = String::new();

                stdin().read_line(&mut input).expect("Failed read");
                if input.len() == 0 {
                    return None;
                } else {
                    self.buffer = input.split_whitespace().rev().map(String::from).collect();
                }
            }
        }

        pub fn many<'a, T: std::str::FromStr>(&'a mut self) -> ScannerManyIterator<'a, T> {
            ScannerManyIterator {
                scanner: self,
                phantom: std::marker::PhantomData,
            }
        }
    }
}
// END UTIL

use util::*;
use std::io::{Write, BufWriter, stdout};
use std::collections::VecDeque;

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut window: VecDeque<i32> = VecDeque::new();
    let mut cur_sum = 0;
    let mut count = 0;

    for next in input.many() {
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
