// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[path = "../util.rs"]
mod util;
// END UTIL

use std::io::{stdout, BufWriter, Write};
use util::*;

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let mut all_strs_oxygen = input.many(EoF).collect::<Vec<String>>();
    let mut all_strs_co2 = all_strs_oxygen.clone();

    for cur_i in 0..all_strs_oxygen[0].len() {
        if all_strs_oxygen.len() > 1 {
            let mut total_count_oxygen = 0;
            let mut counts_oxygen = Vec::new();

            for next in &all_strs_oxygen {
                if counts_oxygen.len() == 0 {
                    counts_oxygen = next.chars().map(|_| 0).collect();
                }

                next.chars().enumerate().for_each(|(i, c)| {
                    if c == '1' {
                        counts_oxygen[i] += 1;
                    }
                });

                total_count_oxygen += 1;
            }

            let mut min_for_max_oxygen = total_count_oxygen / 2;
            if total_count_oxygen % 2 == 1 {
                min_for_max_oxygen += 1;
            }

            all_strs_oxygen = all_strs_oxygen
                .iter()
                .filter(|s| {
                    let expected = if counts_oxygen[cur_i] == min_for_max_oxygen {
                        '1'
                    } else if counts_oxygen[cur_i] >= min_for_max_oxygen {
                        '1'
                    } else {
                        '0'
                    };

                    s.chars().nth(cur_i).unwrap() == expected
                })
                .cloned()
                .collect::<Vec<String>>();
        }

        if all_strs_co2.len() > 1 {
            let mut total_count_co2 = 0;
            let mut counts_co2 = Vec::new();

            for next in &all_strs_co2 {
                if counts_co2.len() == 0 {
                    counts_co2 = next.chars().map(|_| 0).collect();
                }

                next.chars().enumerate().for_each(|(i, c)| {
                    if c == '1' {
                        counts_co2[i] += 1;
                    }
                });

                total_count_co2 += 1;
            }

            let mut min_for_max_co2 = total_count_co2 / 2;
            if total_count_co2 % 2 == 1 {
                min_for_max_co2 += 1;
            }

            all_strs_co2 = all_strs_co2
                .iter()
                .filter(|s| {
                    let expected = if counts_co2[cur_i] == min_for_max_co2 {
                        '0'
                    } else if counts_co2[cur_i] >= min_for_max_co2 {
                        '0'
                    } else {
                        '1'
                    };

                    s.chars().nth(cur_i).unwrap() == expected
                })
                .cloned()
                .collect::<Vec<String>>();
        }
    }

    let best_str_oxygen = &all_strs_oxygen[0];
    let best_str_co2 = &all_strs_co2[0];

    let best_oxygen_parsed = isize::from_str_radix(&best_str_oxygen, 2).unwrap();
    let best_co2_parsed = isize::from_str_radix(&best_str_co2, 2).unwrap();

    writeln!(out, "{}", best_oxygen_parsed * best_co2_parsed).unwrap();
}
