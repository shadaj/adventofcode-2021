// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[path = "../util.rs"]
mod util;
// END UTIL

use std::io::{stdout, BufWriter, Write};
use util::*;

fn neighbors(i: usize, j: usize, grid: &Vec<Vec<u32>>) -> Vec<(u32, (usize, usize))> {
    let mut res = Vec::new();
    for di in -1..=1 {
        for dj in -1..=1 {
            if (di == 0 && dj == 0) || (di * dj) != 0 {
                continue;
            }

            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni >= 0 && ni < grid.len() as isize && nj >= 0 && nj < grid[0].len() as isize {
                res.push((grid[ni as usize][nj as usize], (ni as usize, nj as usize)));
            }
        }
    }

    return res;
}

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let grid: Vec<Vec<u32>> = input
        .many_lines(EoF)
        .map(|line: String| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let neighbors = neighbors(i, j, &grid);
            if neighbors.iter().all(|&(n, _)| n > grid[i][j]) {
                total += 1 + grid[i][j];
            }
        }
    }

    writeln!(out, "{}", total).unwrap();
}
