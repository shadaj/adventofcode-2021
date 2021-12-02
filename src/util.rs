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

#[allow(dead_code)]
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
