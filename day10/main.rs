use std::fs;
use num::Integer;
use std::cmp::Ordering;

fn load_map(s: &str) -> (Vec<Vec<bool>>, Vec<(i64, i64)>) {
    let mut map = Vec::new();
    let mut asteroids = Vec::new();
    for (y, row) in s.lines().enumerate() {
        let mut map_row = Vec::new();
        for (x, cell) in row.chars().enumerate() {
            if cell == '#' {
                asteroids.push((x as i64, y as i64));
                map_row.push(true);
            } else if cell == '.' {
                map_row.push(false);
            } else {
                panic!("Unexpected character '{}' in map", cell);
            }
        }
        map.push(map_row);
    }
    (map, asteroids)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let (map, asteroids) = load_map(&input);

    let mut max_detectable = 0;
    let mut best_x = 0;
    let mut best_y = 0;
    for (i, (x_orig, y_orig)) in asteroids.iter().enumerate() {
        let mut detectable = 0;
        for (j, (x_dest, y_dest)) in asteroids.iter().enumerate() {
            if i != j {
                // check if visible
                let x_diff = x_dest - x_orig;
                let y_diff = y_dest - y_orig;
                let step_count = x_diff.gcd(&y_diff);
                let x_step = x_diff / step_count;
                let y_step = y_diff / step_count;
                let mut collision = false;
                for step in 1..step_count {
                    let x = x_orig + x_step * step;
                    let y = y_orig + y_step * step;
                    if map[y as usize][x as usize] == true {
                        collision = true;
                        break;
                    }
                }
                if !collision {
                    detectable += 1;
                }
            }
        }
        if detectable > max_detectable {
            max_detectable = detectable;
            best_x = *x_orig;
            best_y = *y_orig;
        }
    }

    println!("Part 1: {} from {}, {}", max_detectable, best_x, best_y);

    struct Asteroid {
        x: i64,
        y: i64,
        dist: i64,
        angle: f64,
        vaporized: bool
    };

    impl Ord for Asteroid {
        fn cmp(&self, other: &Self) -> Ordering {
            let primary = self.angle.partial_cmp(&other.angle);
            match primary {
                Some(Ordering::Equal) => self.dist.cmp(&other.dist),
                Some(ordering) => ordering,
                _ => self.dist.cmp(&other.dist)
            }
        }
    }

    impl PartialOrd for Asteroid {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Asteroid {
        fn eq(&self, other: &Self) -> bool {
            self.dist == other.dist && self.angle == other.angle
        }
    }

    impl Eq for Asteroid {}

    let mut vapor = Vec::new();
    for (x, y) in asteroids {
        if x != best_x || y != best_y {
            let dist = (x - best_x).pow(2) + (y - best_y).pow(2);
            let angle = (y as f64 - best_y as f64).atan2(x as f64 - best_x as f64) + std::f64::consts::FRAC_PI_2;
            let angle = if angle < 0.0 { angle + std::f64::consts::PI * 2.0 } else { angle };
            vapor.push(Asteroid { x: x, y: y, dist: dist, angle: angle, vaporized: false });
        }
    }
    vapor.sort();
    let mut vaporized_count = 0;
    let mut prev_angle = -1.0;
    while vaporized_count < 200 {
        for a in &mut vapor {
            if a.vaporized { continue; }
            if a.angle != prev_angle {
                a.vaporized = true;
                vaporized_count += 1;
                if vaporized_count == 200 { println!("Part 2: {}", a.x * 100 + a.y); }
                prev_angle = a.angle;
            }
        }
    }

}