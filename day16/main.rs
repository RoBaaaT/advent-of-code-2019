use std::fs;

fn fft(input: &Vec<i64>) -> Vec<i64> {
    let mut phase_output = input.clone();
    for _phase in 0..100 {
        let phase_input = phase_output;
        phase_output = vec![0; phase_input.len()];
        for input_pos in (0..phase_input.len()).rev() {
            for output_pos in 0..input_pos + 1 {
                let input = phase_input[input_pos];
                match ((input_pos + 1) / (output_pos + 1)) & 3 {
                    0 | 2 => {},
                    1 => phase_output[output_pos] += input,
                    3 => phase_output[output_pos] -= input,
                    _ => panic!()
                }
            }
        }
        for pos in 0..phase_output.len() {
            phase_output[pos] = phase_output[pos].abs() % 10;
        }
    }

    phase_output
}

fn fft_right_half(input: &Vec<i64>) -> Vec<i64> {
    let mut phase_output = input.clone();
    for _phase in 0..100 {
        let phase_input = phase_output;
        phase_output = vec![0; phase_input.len()];
        let mut accu = 0;
        for input_pos in (phase_input.len() / 2..phase_input.len()).rev() {
            accu += phase_input[input_pos];
            phase_output[input_pos] = accu.abs() % 10;
        }
    }

    phase_output
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let message_offset: usize = input[0..7].parse().unwrap();
    let input: Vec<i64> = input.chars().filter_map(|x| x.to_digit(10)).map(|x| x as i64).collect();

    // part 1
    let output = fft(&input);
    print!("Part 1: ");
    output[0..8].iter().for_each(|x| print!("{}", std::char::from_digit(*x as u32, 10).unwrap()));
    println!("");

    // part 2
    let mut full_input = Vec::new();
    for _repeat in 0..10000 {
        full_input.append(&mut input.clone());
    }
    assert!(message_offset >= full_input.len() / 2);
    let output = fft_right_half(&full_input);

    print!("Part 2: ");
    for output_pos in message_offset..message_offset + 8 {
        print!("{}", std::char::from_digit(output[output_pos] as u32, 10).unwrap());
    }
    println!("");
}