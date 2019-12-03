use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let input = BufReader::new(&input_file);
    let mut result1 = 0;
    let mut result2 = 0;
    for line in input.lines() {
        let mass: i64 = line.unwrap().parse().unwrap();
        let fuel = mass / 3 - 2;
        result1 += fuel;
        let mut additional_fuel = fuel;
        while additional_fuel > 0 {
            result2 += additional_fuel;
            additional_fuel = additional_fuel / 3 - 2;
        }
    }
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}