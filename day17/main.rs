use std::fs::File;
use intcode::*;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn tile_in_direction(tiles: &Vec<char>, width: usize, pos: usize, dir: Direction) -> char {
    match dir {
        Direction::Up => if pos < width { '.' } else { tiles[pos - width] },
        Direction::Down => if pos + width >= tiles.len() { '.' } else { tiles[pos + width] },
        Direction::Left => if pos % width == 0 { '.' } else { tiles[pos - 1] },
        Direction::Right => if pos % width == width - 1 { '.' } else { tiles[pos + 1] }
    }
}

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
    let mut start_pos = 0;
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
        } else if *tile == '^' {
            start_pos = i;
        }
    }
    println!("Part 1: {}", alignment_parameters);

    let mut dir = Direction::Up;
    let mut pos = start_pos;
    let mut commands = Vec::new();
    let mut straight_len = 0;
    loop {
        if tile_in_direction(&tiles, width, pos, dir) == '#' {
            straight_len += 1;
            pos = match dir {
                Direction::Up => pos - width,
                Direction::Left => pos - 1,
                Direction::Down => pos + width,
                Direction::Right => pos + 1
            }
        } else {
            if straight_len != 0 {
                commands.push(straight_len.to_string());
                straight_len = 0;
            }
            let (left_dir, right_dir) = match dir {
                Direction::Up => (Direction::Left, Direction::Right),
                Direction::Left => (Direction::Down, Direction::Up),
                Direction::Down => (Direction::Right, Direction::Left),
                Direction::Right => (Direction::Up, Direction::Down)
            };
            if tile_in_direction(&tiles, width, pos, left_dir) == '#' {
                commands.push("L".to_string());
                dir = left_dir;
            } else if tile_in_direction(&tiles, width, pos, right_dir) == '#' {
                commands.push("R".to_string());
                dir = right_dir;
            } else {
                break;
            }
        }
    }

    println!("{:?}", commands);

    tape[0] = 2;
    let command = "A,B,A,B,A,C,B,C,A,C\nL,6,R,12,L,6\nR,12,L,10,L,4,L,6\nL,10,L,10,L,4,L,6\ny\n";
    let mut command_vector = Vec::new();
    for c in command.chars() {
        command_vector.push(c as i64);
    }
    let mut output = StdASCIIOutput::new();
    execute_intcode(&tape, &mut VecInput::new(command_vector), &mut output);
    print!("\n");
    println!("Part 2: {}", output.last_output());
}