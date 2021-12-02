// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[allow(dead_code)]
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
