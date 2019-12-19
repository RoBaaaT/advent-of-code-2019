use std::fs::File;
use intcode::*;

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let mut tape = load_tape(input_file);

    let mut output = VecOutput::new();
    execute_intcode(&tape, &mut StdInput, &mut output);
    let mut width = 0;
    let mut tiles = Vec::new();
    for (i, tile) in output.values().iter().enumerate() {
        let printable_tile = *tile as u8 as char;
        print!("{}", printable_tile);

        if printable_tile == '\n' {
            if width == 0 {
                width = i;
            } else if i != output.values().len() - 1 {
                assert_eq!((i + 1) % (width + 1), 0);
            }
        } else {
            tiles.push(printable_tile);
        }
    }

    let mut alignment_parameters = 0;
    let height = tiles.len() / width;
    for (i, tile) in tiles.iter().enumerate() {
        if *tile == '#' {
            let x = i % width;
            let y = i / width;
            if x > 0 && x < width - 1 && y > 0 && y < height - 1 {
                if tiles[i - 1] == '#' && tiles[i + 1] == '#' && tiles[i - width] == '#' && tiles[i + width] == '#' {
                    let alignment_parameter = x * y;
                    alignment_parameters += alignment_parameter;
                }
            }
        }
    }
    println!("Part 1: {}", alignment_parameters);

    tape[0] = 2;
    execute_intcode(&tape, &mut StdInput, &mut StdASCIIOutput);
}