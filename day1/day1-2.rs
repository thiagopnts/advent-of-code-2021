// rustc day1-2.rs && ./day1-2 < input.txt
use std::io::{self, BufRead};

fn main() {
    println!(
        "{}",
        io::stdin()
            .lock()
            .lines()
            .take_while(|line| line.is_ok())
            .map(|result| result.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .as_slice()
            .windows(3)
            .map(|win| win[0] + win[1] + win[2])
            .collect::<Vec<_>>()
            .as_slice()
            .windows(2)
            .filter(|win| win[1] > win[0])
            .collect::<Vec<_>>()
            .len()
    );
}
