use std::fs::File;
use intcode::*;

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let tape = load_tape(input_file);

    let springcode = "NOT A T\nOR T J\nNOT B T\nOR T J\nNOT C T\nOR T J\nAND D J\nWALK\n";
    let mut input = StringInput::new(springcode.chars());
    let mut output = StdASCIIOutput::new();
    execute_intcode(&tape, &mut input, &mut output);
    print!("\n");
    println!("Part 1: {}", output.last_output());

    let springcode = "NOT A J
        NOT C T
        AND H T
        OR T J
        NOT B T
        AND A T
        AND C T
        OR T J
        AND D J
        RUN
        ";
    let mut input = StringInput::new(springcode.chars());
    let mut output = StdASCIIOutput::new();
    execute_intcode(&tape, &mut input, &mut output);
    print!("\n");
    println!("Part 2: {}", output.last_output());
}