// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[path = "../util.rs"]
mod util;
// END UTIL

use std::{
    collections::{hash_map::RandomState, HashSet},
    io::{stdout, BufWriter, Write},
};
use util::*;

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut entries: Vec<(Vec<String>, Vec<String>)> = vec![];
    while let Ok(entry) = input.next_line::<String>() {
        let (before, after) = entry.split_once(" | ").unwrap();
        entries.push((
            before.split_whitespace().map(|s| s.to_string()).collect(),
            after.split_whitespace().map(|s| s.to_string()).collect(),
        ));
    }

    let mut sum = 0;
    for (before, after) in entries {
        let mut in_one = None;
        let mut in_four = None;
        let mut in_seven = None;
        let mut in_eight = None;
        let mut in_2_3_5 = HashSet::new();
        let mut in_0_6_9 = HashSet::new();

        for v in before {
            let len = v.len();
            match len {
                2 => in_one = Some(v),
                4 => in_four = Some(v),
                3 => in_seven = Some(v),
                5 => {
                    in_2_3_5.insert(v);
                }
                6 => {
                    in_0_6_9.insert(v);
                }
                7 => in_eight = Some(v),
                _ => panic!("unexpected length"),
            }
        }

        let cf: HashSet<char, RandomState> = HashSet::from_iter(in_one.unwrap().chars());
        let bcdf: HashSet<char, RandomState> = HashSet::from_iter(in_four.unwrap().chars());
        let acf: HashSet<char, RandomState> = HashSet::from_iter(in_seven.unwrap().chars());
        let abcdefg: HashSet<char, RandomState> = HashSet::from_iter(in_eight.unwrap().chars());
        let bd: HashSet<char, RandomState> = bcdf.difference(&cf).cloned().collect();
        let aeg: HashSet<char, RandomState> = abcdefg.difference(&bcdf).cloned().collect();
        let adg = in_2_3_5
            .iter()
            .map(|f| HashSet::<char, RandomState>::from_iter(f.chars()))
            .reduce(|a, b| a.intersection(&b).cloned().collect())
            .unwrap();
        let abgf = in_0_6_9
            .iter()
            .map(|f| HashSet::<char, RandomState>::from_iter(f.chars()))
            .reduce(|a, b| a.intersection(&b).cloned().collect())
            .unwrap();
        let ag: HashSet<char, RandomState> = aeg.intersection(&adg).cloned().collect();

        let a_char = *acf.difference(&cf).next().unwrap();
        let g_char = *ag.difference(&HashSet::from([a_char])).next().unwrap();
        let e_char = *aeg
            .difference(&HashSet::from([a_char, g_char]))
            .next()
            .unwrap();
        let d_char = *adg
            .difference(&HashSet::from([a_char, g_char]))
            .next()
            .unwrap();
        let b_char = *bd.difference(&HashSet::from([d_char])).next().unwrap();
        let f_char = *abgf
            .difference(&HashSet::from([a_char, b_char, g_char]))
            .next()
            .unwrap();
        let c_char = *bcdf
            .difference(&HashSet::from([b_char, d_char, f_char]))
            .next()
            .unwrap();

        let mapping: Vec<(HashSet<char>, u32)> = vec![
            (
                HashSet::from([a_char, b_char, c_char, e_char, f_char, g_char]),
                0,
            ),
            (HashSet::from([c_char, f_char]), 1),
            (HashSet::from([a_char, c_char, d_char, e_char, g_char]), 2),
            (HashSet::from([a_char, c_char, d_char, f_char, g_char]), 3),
            (HashSet::from([b_char, c_char, d_char, f_char]), 4),
            (HashSet::from([a_char, b_char, d_char, f_char, g_char]), 5),
            (
                HashSet::from([a_char, b_char, d_char, e_char, f_char, g_char]),
                6,
            ),
            (HashSet::from([a_char, c_char, f_char]), 7),
            (
                HashSet::from([a_char, b_char, c_char, d_char, e_char, f_char, g_char]),
                8,
            ),
            (
                HashSet::from([a_char, b_char, c_char, d_char, f_char, g_char]),
                9,
            ),
        ];

        let mut output = 0;
        for digit in &after {
            output *= 10;

            let mut chars = HashSet::new();
            for c in digit.chars() {
                chars.insert(c);
            }

            for (set, digit) in &mapping {
                if set == &chars {
                    output += digit;
                    break;
                }
            }
        }

        sum += output;
    }

    writeln!(out, "{}", sum).unwrap();
}
