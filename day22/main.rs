use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let input = BufReader::new(&input_file);
    const SIZE: i64 = 10007;
    let mut position: i64 = 2019;
    for line in input.lines() {
        let line = line.unwrap();
        match line.as_ref() {
            "deal into new stack" => {
                position = SIZE - 1 - position;
            },
            x if x.starts_with("cut ") => {
                let cut_value: i64 = x.split("cut ").last().unwrap().parse().unwrap();
                position = (position - cut_value) % SIZE;
            },
            x if x.starts_with("deal with increment ") => {
                let increment: i64 = x.split("deal with increment ").last().unwrap().parse().unwrap();
                position = (position * increment) % SIZE;
            },
            _ => panic!("unexpected input: {}", line)
        }
    }

    println!("Part 1: {}", position);
}