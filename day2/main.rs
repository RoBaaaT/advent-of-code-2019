use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use intcode::execute_intcode;

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let input = BufReader::new(&input_file);
    let mut tape = Vec::new();
    for opcode in input.split(b',') {
        let op_str = String::from_utf8(opcode.unwrap()).unwrap();
        let op: i64 = op_str.trim_end().parse().unwrap();
        tape.push(op);
    }

    tape[1] = 12;
    tape[2] = 2;

    println!("Part 1: {}", execute_intcode(&tape)[0]);

    for noun in 0..99 {
        for verb in 0..99 {       
            tape[1] = noun;
            tape[2] = verb;
            if execute_intcode(&tape)[0] == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
            }
        }
    }
}