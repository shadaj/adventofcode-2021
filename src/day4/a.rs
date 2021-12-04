// BEGIN UTIL (https://codeforces.com/blog/entry/67391)
#[path = "../util.rs"]
mod util;
// END UTIL

use std::io::{stdout, BufWriter, Write};
use util::*;

fn main() {
    let mut input = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let picked_nums = input.many_by(",", IsWhitespace).collect::<Vec<i32>>();

    let boards = input
        .many(EoF)
        .collect::<Vec<i32>>()
        .chunks(5 * 5)
        .map(|c| c.iter().cloned().collect())
        .collect::<Vec<Vec<i32>>>();

    let mut boards_marked = vec![vec![false; 25]; boards.len()];

    fn board_won(board: &Vec<bool>) -> bool {
        for column in 0..5 {
            if (0..5).all(|row| board[row * 5 + column]) {
                return true;
            }
        }

        for row in 0..5 {
            if (0..5).all(|column| board[row * 5 + column]) {
                return true;
            }
        }

        return false;
    }

    fn update_board(value: i32, board_nums: &Vec<i32>, board: &mut Vec<bool>) {
        for (i, &num) in board_nums.iter().enumerate() {
            if num == value {
                board[i] = true;
            }
        }
    }

    for num in picked_nums {
        for i in 0..boards.len() {
            update_board(num, &boards[i], &mut boards_marked[i]);
        }

        for board_i in 0..boards.len() {
            if board_won(&boards_marked[board_i]) {
                let unmarked: i32 = boards_marked[board_i]
                    .iter()
                    .enumerate()
                    .map(|(i, &x)| if !x { boards[board_i][i] } else { 0 })
                    .sum();

                writeln!(out, "{}", unmarked * num).unwrap();
                return;
            }
        }
    }
}
