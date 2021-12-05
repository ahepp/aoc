use std::fs;
use std::io;
use std::io::prelude::*;

const BOARD_ROWS: usize = 5;
const BOARD_COLS: usize = 5;

type Board = Vec<BoardRow>;
type BoardRow = Vec<BoardCell>;
type BoardCell = (u32, bool);

fn main() -> io::Result<()> {
    let text = fs::read_to_string("input.txt")?;
    let mut lines = text.trim().split('\n');

    let drawns: Vec<u32> = lines.next().unwrap()
        .trim().split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{:?}", drawns);

    let mut boards: Vec<Board> = lines
        .filter(|l| !l.is_empty())
        .map(|l| l.split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| (s.parse().unwrap(), false))
            .collect())
        .collect::<Board>()
        .chunks(BOARD_ROWS)
        .map(|s| s.into())
        .collect();

    let mut solved: Vec<bool> = vec![false; boards.len()];

    //print_boards(&boards);

    for drawn in drawns {
        for (board_idx, board) in boards.iter_mut().enumerate() {
            let (row_idx, col_idx) = mark_board(board, drawn);
            if check_row(&board, row_idx) || check_col(&board, col_idx) {
                solved[board_idx] = true;
                println!("{:?}", solved);
                let sum = score_board(&board);
                let score = sum * drawn;
                println!("Board Sum: {}", sum);
                println!("Score: {}", score);
                //print_boards(&boards);
                if(solved.iter().all(|b| *b == true)) {
                    println!("Last board solved");
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

fn print_boards(boards: &Vec<Board>) {
    for board in boards {
        println!();
        for row in board {
            println!("{:?}", row);
        }
    }
}

fn mark_board(board: &mut Board, drawn: u32) -> (usize, usize) {
    for (row_idx, row) in board.iter_mut().enumerate() {
        for (col_idx, (val, hit)) in row.iter_mut().enumerate() {
            if val.clone() == drawn {
                *hit = true;
                return (row_idx, col_idx);
            }
        }
    }
    return (0,0)
}

fn check_row(board: &Board, row_idx: usize) -> bool {
    let mut all_true = true;
    for (_, hit) in &board[row_idx] {
        all_true = all_true & hit;
    }
    println!("check row {}", all_true);
    return all_true;
}

fn check_col(board: &Board, col_idx: usize) -> bool {
    let mut all_true = true;
    for row in board {
        let (_, hit) = row[col_idx];
        all_true = all_true & hit;
    }
    println!("check col {}", all_true);
    return all_true;
}

fn score_board(board: &Board) -> u32 {
    let mut sum = 0;
    for row in board {
        for (val, hit) in row {
            if !*hit {
                sum = sum + val;
            }
        }
    }
    return sum;
}
