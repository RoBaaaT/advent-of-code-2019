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
        let mut map = vec![false; MAP_SIZE * MAP_SIZE];
        let mut x = ORIGIN;
        let mut y = ORIGIN;
        for entry in line.unwrap().split(",") {
            let direction = &entry[0..1];
            let distance: usize = entry[1..entry.len()].parse().unwrap();
            match direction {
                "L" => {
                    for offset in 0..distance { map[y * MAP_SIZE + x - 1 - offset] = true; }
                    x -= distance
                },
                "R" => {
                    for offset in 0..distance { map[y * MAP_SIZE + x + 1 + offset] = true; }
                    x += distance
                },
                "D" => {
                    for offset in 0..distance { map[(y - 1 - offset) * MAP_SIZE + x] = true; }
                    y -= distance
                },
                "U" => {
                    for offset in 0..distance { map[(y + 1 + offset) * MAP_SIZE + x] = true; }
                    y += distance
                },
                _ => panic!("invalid direction code: {}", direction)
            }
        }
        maps.push(map);
    }
    let mut min_distance = std::usize::MAX;
    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            if maps[0][y * MAP_SIZE + x] == true && maps[1][y * MAP_SIZE + x] == true {
                let distance = if x > ORIGIN { x - ORIGIN } else { ORIGIN - x }
                    + if y > ORIGIN { y - ORIGIN } else { ORIGIN - y };
                if distance != 0 && distance < min_distance {
                    min_distance = distance;
                }
            }
        }
    }
    println!("Part 1: {}", min_distance);
}