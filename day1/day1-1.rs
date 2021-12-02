// rustc day1-1.rs && ./day1-1 < input.txt
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
            .windows(2)
            .filter(|win| win[1] > win[0])
            .collect::<Vec<_>>()
            .len()
    );
}
