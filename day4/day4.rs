// rustc day4.rs && ./day4 < input.txt
use std::collections::VecDeque;
use std::io::{self, BufRead};

fn is_complete(board: &[Vec<(i32, bool)>]) -> bool {
    for i in 0..5 {
        if (0..5).map(|j| board[i][j].1).all(|m| m) || (0..5).map(|j| board[j][i].1).all(|m| m) {
            return true;
        }
    }
    false
}

fn unmarked_sum(board: &[Vec<(i32, bool)>]) -> i32 {
    return board
        .iter()
        .flatten()
        .filter(|n| !n.1)
        .map(|n| n.0)
        .collect::<Vec<_>>()
        .iter()
        .sum::<i32>();
}

fn main() {
    let mut draws: VecDeque<i32> = io::stdin()
        .lock()
        .lines()
        .take(1)
        .map(|line| {
            let line = line.unwrap();
            line.split(',')
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .flatten()
        .collect::<VecDeque<i32>>();

    let mut boards = vec![];

    loop {
        let board = io::stdin()
            .lock()
            .lines()
            .take_while(|line| line.is_ok())
            .skip_while(|line| line.as_ref().unwrap().clone().is_empty())
            .take(5)
            .map(|r| r.unwrap())
            .map(|line| {
                line.split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|c| (c.parse::<i32>().unwrap(), false))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<(i32, bool)>>>();

        if board.is_empty() {
            break;
        }
        boards.push(board);
    }

    let mut done = vec![];
    let total = boards.clone().len();
    'main: while let Some(draw) = draws.pop_front() {
        for (id, board) in boards.iter_mut().enumerate() {
            for row in board.iter_mut() {
                for number in row.iter_mut() {
                    if number.0 == draw {
                        println!("{} marked on board {}", number.0, id);
                        number.1 = true;
                    }
                }
            }
            if is_complete(board) && !done.contains(&id) {
                println!("board {} marked as done", id);
                done.push(id);
                let unmarked_sum = unmarked_sum(board);
                if done.len() == 1 {
                    println!(
                        "part 1 solution last draw: {}, unmarked_sum: {} = {}",
                        draw,
                        unmarked_sum,
                        draw * unmarked_sum
                    );
                }
                if done.len() == total {
                    println!(
                        "part 2 solution last draw: {}, unmarked_sum: {} = {}",
                        draw,
                        unmarked_sum,
                        draw * unmarked_sum,
                    );
                    break 'main;
                }
            }
        }
    }
}
