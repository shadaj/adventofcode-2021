// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[path = "../util.rs"]
mod util;
// END UTIL

use std::{io::{stdout, BufWriter, Write}};
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

fn mark_pool(i: usize, j: usize, grid: &Vec<Vec<u32>>, areas: &mut Vec<Vec<bool>>) -> u32 {
    if areas[i][j] || grid[i][j] == 9 {
        return 0;
    }

    areas[i][j] = true;

    return neighbors(i, j, grid)
        .iter()
        .map(|&(_, (ni, nj))| mark_pool(ni, nj, grid, areas))
        .sum::<u32>()
        + 1;
}

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let grid: Vec<Vec<u32>> = input
        .many_lines(EoF)
        .map(|line: String| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut areas: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut counts = vec![];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            counts.push(mark_pool(i, j, &grid, &mut areas));
        }
    }

    counts.sort();
    let mul_3 = counts.iter().rev().take(3).product::<u32>();

    writeln!(out, "{}", mul_3).unwrap();
}
