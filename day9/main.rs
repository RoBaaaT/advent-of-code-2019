use std::fs::File;
use intcode::*;

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let tape = load_tape(input_file);

    println!("Part 1:");
    execute_intcode(&tape, &mut StdInput, &mut StdOutput);
}