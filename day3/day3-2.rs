// rustc day3-2.rs && ./day3-2 < input.txt
use std::io::{self, BufRead};
use std::iter;

fn process(
    lines: Vec<Vec<char>>,
    counts_by_column: Vec<(i32, i32)>,
    criteria: fn(i32, i32) -> char,
) -> i64 {
    i64::from_str_radix(
        _process(lines.clone(), counts_by_column.clone(), 0, criteria)[0]
            .iter()
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap()
}

fn _process(
    lines: Vec<Vec<char>>,
    counts_by_column: Vec<(i32, i32)>,
    idx: usize,
    criteria: fn(i32, i32) -> char,
) -> Vec<Vec<char>> {
    if lines.len() == 1 {
        return lines;
    }
    let (zeros, ones) = counts_by_column[idx];
    let mut values = vec![];
    let bit_criteria = criteria(zeros, ones);
    for line in lines.iter() {
        if line[idx] == bit_criteria {
            values.push(line.clone());
        }
    }
    let counts_by_column = count_bits_columns(values.clone());
    return _process(values.clone(), counts_by_column.clone(), idx + 1, criteria);
}

fn count_bits_columns(lines: Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut result = iter::repeat((0, 0))
        .take(lines[0].len())
        .collect::<Vec<_>>();
    for (i, _) in lines[0].iter().enumerate() {
        for (j, _) in lines.iter().enumerate() {
            match lines[j][i] {
                '0' => result[i].0 += 1,
                '1' => result[i].1 += 1,
                _ => panic!("wat"),
            }
        }
    }
    result
}

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .take_while(|line| line.is_ok())
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let counts_by_column = count_bits_columns(lines.clone());
    let oxi = process(lines.clone(), counts_by_column.clone(), |zeros, ones| {
        if ones >= zeros {
            '1'
        } else {
            '0'
        }
    });

    let co2 = process(lines.clone(), counts_by_column.clone(), |zeros, ones| {
        if zeros <= ones {
            '0'
        } else {
            '1'
        }
    });
    println!("{}", oxi * co2);
}
