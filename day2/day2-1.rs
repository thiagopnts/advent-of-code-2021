use std::io::{self, BufRead};

fn main() {
    println!(
        "{}",
        std::io::stdin()
            .lock()
            .lines()
            .take_while(|line| line.is_ok())
            .map::<Vec<_>, _>(|line| line.unwrap().split(' ').map(|s| s.to_owned()).collect())
            .map::<(String, _), _>(|v| (v[0].clone(), v[1].parse::<i32>().unwrap()))
            .fold(vec![0, 0], |mut pos, direction| {
                match direction.0.as_str() {
                    "up" => pos[0] -= direction.1,
                    "down" => pos[0] += direction.1,
                    _ => pos[1] += direction.1,
                };
                pos
            })
            .into_iter()
            .reduce::<_>(|depth, hor| depth * hor)
            .unwrap()
    );
}
