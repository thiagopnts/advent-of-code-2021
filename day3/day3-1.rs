// rustc day3-1.rs && ./day3-1 < input.txt
use std::io::{self, BufRead};
use std::iter;

fn main() {
    println!(
        "{}",
        io::stdin()
            .lock()
            .lines()
            .take_while(|line| line.is_ok())
            .map(|line| line.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<char>>>()
            .iter()
            .fold::<Option<Vec<_>>, _>(Some(vec![]), |lines, line| {
                let mut lines = lines.unwrap();
                lines.push(line);
                Some(lines)
            })
            .map(|lines| {
                return lines
                    .iter()
                    .enumerate()
                    .fold::<Vec<(i32, i32)>, _>(
                        iter::repeat((0, 0))
                            .take(lines[0].len())
                            .collect::<Vec<_>>(),
                        |mut result, (_, line)| {
                            line.iter().enumerate().for_each(|(j, c)| {
                                match c {
                                    '0' => result[j].0 += 1,
                                    '1' => result[j].1 += 1,
                                    _ => panic!("wat"),
                                };
                            });
                            return result;
                        },
                    )
                    .iter()
                    .map(|(zeros, ones)| (ones > zeros, ones < zeros))
                    .map(|(g, e)| ((g as i32).to_string(), (e as i32).to_string()))
                    .fold::<Vec<(String, String)>, _>(
                        vec![("".to_owned(), "".to_owned())],
                        |mut v, (g, e)| {
                            v[0].0 += &g;
                            v[0].1 += &e;
                            v
                        },
                    )
                    .iter()
                    .map(|(g, e)| {
                        (
                            i32::from_str_radix(g, 2).unwrap(),
                            i32::from_str_radix(e, 2).unwrap(),
                        )
                    })
                    .map(|(g, e)| g * e)
                    .collect::<Vec<_>>();
            })
            .unwrap()[0]
    );
}
