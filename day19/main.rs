use std::fs::File;
use intcode::*;


fn main() {
    let input_file = File::open("input.txt").unwrap();
    let tape = load_tape(input_file);

    let mut affected_count = 0;

    for x in 0..50 {
        for y in 0..50 {
            let mut output = VecOutput::new();
            let mut input = VecInput::new(vec![x, y]);
            execute_intcode(&tape, &mut input, &mut output);
            affected_count += output.values()[0];
        }
    }
    println!("Part 1: {}", affected_count);

    let mut found = false;
    let mut found_x = 0;
    let mut found_y = 0;
    let mut dist = 900; // start fairly far away, a faster way to do this would be to perform a binary search on dist
    let mut start_i = 0;
    const SIZE: i64 = 100;
    while !found {
        let mut top_left_matched = false;
        for i in start_i..dist * 2 + 1 {
            let x = if i < dist { dist } else { 2 * dist - i };
            let y = if i > dist { dist } else { i };
            let r_x = x + SIZE - 1;
            let b_y = y + SIZE - 1;
            let mut output = VecOutput::new();
            let mut input = VecInput::new(vec![x, y, x, b_y, r_x, y, r_x, b_y]);
            execute_intcode(&tape, &mut input, &mut output);
            execute_intcode(&tape, &mut input, &mut output);
            execute_intcode(&tape, &mut input, &mut output);
            execute_intcode(&tape, &mut input, &mut output);
            if output.values()[0] == 1 && output.values()[1] == 1 && output.values()[2] == 1 && output.values()[3] == 1 {
                found = true;
                found_x = x;
                found_y = y;
                break;
            }
            if output.values()[0] == 1 && !top_left_matched {
                top_left_matched = true;
                start_i = i;
            }
        }
        dist += 1;
    }
    println!("Part 2: {}", found_x * 10000 + found_y);
}