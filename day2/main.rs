use std::fs::File;
use intcode::*;

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let mut tape = load_tape(input_file);

    tape[1] = 12;
    tape[2] = 2;

    println!("Part 1: {}", execute_intcode(&tape, &mut StdInput, &mut StdOutput)[0]);

    for noun in 0..99 {
        for verb in 0..99 {       
            tape[1] = noun;
            tape[2] = verb;
            if execute_intcode(&tape, &mut StdInput, &mut StdOutput)[0] == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
            }
        }
    }
}