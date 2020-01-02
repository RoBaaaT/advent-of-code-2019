use std::fs::File;
use intcode::*;

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let tape = load_tape(input_file);

    let mut input = StdASCIIInput::new();
    let mut output = StdASCIIOutput::new();
    execute_intcode(&tape, &mut input, &mut output);
}