// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
use std::{collections::VecDeque, io::{stdin, stdout, BufWriter, Write}};
#[allow(unused_imports)]
use std::{collections::HashSet, writeln};

struct Scanner {
  buffer: Vec<String>,
}

impl Scanner {
  fn new() -> Scanner {
    Scanner { buffer: Vec::new() }
  }

  fn next<T: std::str::FromStr>(&mut self) -> Option<T> {
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
}

// END UTIL

fn main() {
  let mut input = Scanner::new();
  let out = &mut BufWriter::new(stdout());

  let mut window: VecDeque<i32> = VecDeque::new();
  let mut cur_sum = 0;
  let mut count = 0;

  loop {
    if let Some(next) = input.next() {
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
    } else {
      break;
    }
  }

  writeln!(out, "{}", count).unwrap();
}
