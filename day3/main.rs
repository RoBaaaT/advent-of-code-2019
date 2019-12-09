use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let input = BufReader::new(&input_file);
    let mut maps = Vec::new();
    const SCALE: usize = 4;
    const MAP_SIZE: usize = 8192 * SCALE;
    const ORIGIN: usize = 4096 * SCALE;
    for line in input.lines() {
        let mut map = vec![0; MAP_SIZE * MAP_SIZE];
        let mut steps = 1;
        let mut x = ORIGIN;
        let mut y = ORIGIN;
        for entry in line.unwrap().split(",") {
            let direction = &entry[0..1];
            let distance: usize = entry[1..entry.len()].parse().unwrap();
            match direction {
                "L" => {
                    for offset in 0..distance {
                        if map[y * MAP_SIZE + x - 1 - offset] == 0 { map[y * MAP_SIZE + x - 1 - offset] = steps; }
                        steps += 1;
                    }
                    x -= distance;
                },
                "R" => {
                    for offset in 0..distance {
                        if map[y * MAP_SIZE + x + 1 + offset] == 0 { map[y * MAP_SIZE + x + 1 + offset] = steps; }
                        steps += 1;
                    }
                    x += distance;
                },
                "D" => {
                    for offset in 0..distance {
                        if map[(y - 1 - offset) * MAP_SIZE + x] == 0 { map[(y - 1 - offset) * MAP_SIZE + x] = steps; }
                        steps += 1;
                    }
                    y -= distance;
                },
                "U" => {
                    for offset in 0..distance {
                        if map[(y + 1 + offset) * MAP_SIZE + x] == 0 { map[(y + 1 + offset) * MAP_SIZE + x] = steps; }
                        steps += 1;
                    }
                    y += distance;
                },
                _ => panic!("invalid direction code: {}", direction)
            }
        }
        maps.push(map);
    }
    let mut min_distance = std::usize::MAX;
    let mut min_steps = std::usize::MAX;
    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            if maps[0][y * MAP_SIZE + x] != 0 && maps[1][y * MAP_SIZE + x] != 0 {
                let distance = if x > ORIGIN { x - ORIGIN } else { ORIGIN - x }
                    + if y > ORIGIN { y - ORIGIN } else { ORIGIN - y };
                let steps = maps[0][y * MAP_SIZE + x] + maps[1][y * MAP_SIZE + x];
                if distance != 0 && distance < min_distance {
                    min_distance = distance;
                }
                if steps != 0 && steps < min_steps {
                    min_steps = steps;
                }
            }
        }
    }
    println!("Part 1: {}", min_distance);
    println!("Part 2: {}", min_steps);
}