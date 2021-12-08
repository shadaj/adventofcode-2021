use std::{io::stdin, str::FromStr};

pub trait NotPattern {
    fn run_find(&self, text: &str) -> Option<usize>;
    fn run_starts_with(&self, text: &str) -> bool;
}

impl NotPattern for &str {
    fn run_find(&self, text: &str) -> Option<usize> {
        text.find(self)
    }

    fn run_starts_with(&self, text: &str) -> bool {
        text.starts_with(self)
    }
}

impl<F: Fn(char) -> bool> NotPattern for F {
    fn run_find(&self, text: &str) -> Option<usize> {
        text.find(self)
    }

    fn run_starts_with(&self, text: &str) -> bool {
        text.starts_with(self)
    }
}

pub struct IsWhitespace;
impl NotPattern for IsWhitespace {
    fn run_find(&self, text: &str) -> Option<usize> {
        text.find(char::is_whitespace)
    }

    fn run_starts_with(&self, text: &str) -> bool {
        text.starts_with(char::is_whitespace)
    }
}

pub struct EoF;
impl NotPattern for EoF {
    fn run_find(&self, text: &str) -> Option<usize> {
        text.find(|_| false)
    }

    fn run_starts_with(&self, text: &str) -> bool {
        text.starts_with(|_| false)
    }
}

pub struct ScannerManyIterator<'a, T: FromStr, P: NotPattern, P2: NotPattern> {
    scanner: &'a mut Scanner,
    delim: P,
    stop: P2,
    is_done: bool,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T: FromStr, P: NotPattern, P2: NotPattern> Iterator for ScannerManyIterator<'a, T, P, P2> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if !self.is_done {
            match self.scanner.next_by(&self.delim) {
                Ok(r) => Some(r),
                Err(_) => {
                    self.is_done = true;
                    self.scanner.next_by(&self.stop).ok()
                }
            }
        } else {
            None
        }
    }
}

pub struct Scanner {
    buffer: String,
    cur_index: usize,
}

#[allow(dead_code)]
impl Scanner {
    pub fn new() -> Scanner {
        let buf = String::new();
        Scanner {
            buffer: buf,
            cur_index: 0,
        }
    }

    pub fn next_by<T: std::str::FromStr, P: NotPattern>(
        &mut self,
        delim: &P,
    ) -> Result<T, Option<T::Err>> {
        let mut acc_cur_index = self.cur_index;
        loop {
            let cur_slice = &self.buffer[acc_cur_index..];
            if let Some(mut index_of_split) = delim.run_find(cur_slice) {
                let end_of_tok = index_of_split;
                acc_cur_index += index_of_split + 1;
                while delim.run_starts_with(&cur_slice[index_of_split..]) {
                    index_of_split += delim.run_find(&cur_slice[index_of_split..]).unwrap() + 1;
                }

                if end_of_tok > 0 {
                    let parsed = cur_slice[..end_of_tok].parse();
                    if parsed.is_ok() {
                        self.cur_index = acc_cur_index;
                    }

                    return parsed.map_err(|e| Some(e));
                }
            } else {
                if cur_slice.len() == 0 {
                    self.buffer = String::new();
                    self.cur_index = 0;
                    acc_cur_index = 0;
                }

                let additional = stdin().read_line(&mut self.buffer).expect("Failed read");

                if additional == 0 {
                    let cur_slice = &self.buffer[acc_cur_index..];
                    if cur_slice.len() == 0 {
                        return Err(None); // reached EoF
                    } else {
                        let parsed = cur_slice
                            .parse()
                            .map_err(|e| Some(e))
                            .or(cur_slice.trim().parse().map_err(|e| Some(e)));

                        if parsed.is_ok() {
                            self.buffer = String::new();
                            self.cur_index = 0;
                        }

                        return parsed;
                    }
                }
            }
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> Result<T, Option<T::Err>> {
        self.next_by(&IsWhitespace)
    }

    pub fn next_line<T: std::str::FromStr>(&mut self) -> Result<T, Option<T::Err>> {
        self.next_by(&"\n")
    }

    pub fn many<'a, T: std::str::FromStr, P2: NotPattern>(
        &'a mut self,
        stop: P2,
    ) -> ScannerManyIterator<'a, T, IsWhitespace, P2> {
        self.many_by(IsWhitespace, stop)
    }

    pub fn many_lines<'a, T: std::str::FromStr, P2: NotPattern>(
        &'a mut self,
        stop: P2,
    ) -> ScannerManyIterator<'a, T, &str, P2> {
        self.many_by(&"\n", stop)
    }

    pub fn many_by<'a, T: std::str::FromStr, P: NotPattern, P2: NotPattern>(
        &'a mut self,
        delim: P,
        stop: P2,
    ) -> ScannerManyIterator<'a, T, P, P2> {
        ScannerManyIterator {
            scanner: self,
            delim,
            stop,
            is_done: false,
            phantom: std::marker::PhantomData,
        }
    }
}
