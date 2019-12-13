use std::fs;
use num::Integer;

#[derive(Debug, Clone, PartialEq)]
struct MoonCoordinate {
    p: i64,
    v: i64
}

#[derive(Debug, Clone, PartialEq)]
struct Moon {
    coords: [MoonCoordinate; 3]
}

fn coord_simulation_step(moons: &mut Vec<Moon>, coord: usize) {
    for i in 0..moons.len() {
        for j in i + 1..moons.len() {
            if moons[i].coords[coord].p < moons[j].coords[coord].p {
                moons[i].coords[coord].v += 1;
                moons[j].coords[coord].v -= 1;
            } else if moons[i].coords[coord].p > moons[j].coords[coord].p {
                moons[i].coords[coord].v -= 1;
                moons[j].coords[coord].v += 1;
            }
        }
    }

    // update position
    for moon in moons.iter_mut() {
        moon.coords[coord].p += moon.coords[coord].v;
    }
}

fn simulation_step(moons: &mut Vec<Moon>) {
    // update velocity
    for l in 0..3 {
        coord_simulation_step(moons, l);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut moons = Vec::new();
    for moon in input.lines() {
        assert_eq!(&moon[0..1], "<");
        assert_eq!(&moon[moon.len() - 1..moon.len()], ">");
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        for coord in moon[1..moon.len() - 1].split(',') {
            let coord_parts: Vec<&str> = coord.trim().split('=').collect();
            assert_eq!(coord_parts.len(), 2);
            match coord_parts[0] {
                "x" => x = coord_parts[1].parse().unwrap(),
                "y" => y = coord_parts[1].parse().unwrap(),
                "z" => z = coord_parts[1].parse().unwrap(),
                _ => panic!("invalid coordinate provided")
            }
        }
        moons.push(Moon { coords: [MoonCoordinate { p: x, v: 0 }, MoonCoordinate { p: y, v: 0 }, MoonCoordinate { p: z, v: 0 }] });
    }
    let mut moons_part2 = moons.clone();

    for _ in 0..1000 {
        simulation_step(&mut moons);
    }

    // calculate energy
    let mut energy = 0;
    for moon in &moons {
        let mut pot = 0;
        let mut kin = 0;
        for coord in 0..3 {
            pot += moon.coords[coord].p.abs();
            kin += moon.coords[coord].v.abs();
        }
        energy += pot * kin;
    }

    println!("Part 1: {}", energy);

    let initial_state = moons_part2.clone();
    let mut repeat_interval = [0u64; 3];
    for coord in 0..3 {
        loop {
            coord_simulation_step(&mut moons_part2, coord);
            repeat_interval[coord] += 1;
            let mut equal = true;
            for i in 0..moons_part2.len() {
                if moons_part2[i].coords[coord] != initial_state[i].coords[coord] {
                    equal = false;
                }
            }
            if equal {
                break;
            }
        }
    }

    println!("Part 2: {:?}", repeat_interval[0].lcm(&repeat_interval[1]).lcm(&repeat_interval[2]));
}