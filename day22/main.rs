use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Technique {
    DealIntoNewStack,
    Cut(i64),
    DealWithIncrement(i64),
}

fn multiplicative_inverse(a: i128, n: i128) -> i128 {
    let mut t = 0i128;
    let mut newt = 1i128;
    let mut r = n;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;
        t = t - quotient * newt;
        r = r - quotient * newr;
        std::mem::swap(&mut t, &mut newt);
        std::mem::swap(&mut r, &mut newr);
    }

    if r > 1 {
        panic!("invalid n");
    }
    if t < 0 {
        t += n;
    }

    t
}

fn get_mul_add_to_reverse_shuffle(steps: &[Technique], deck_size: i128) -> (i128, i128) {
    let mut mul = 1i128;
    let mut add = 0i128;
    for &step in steps.iter().rev() {
        match step {
            Technique::DealIntoNewStack => {
                add += 1;
                let x = deck_size - 1;
                mul = (mul * x) % deck_size;
                add = (add * x) % deck_size;
            }
            Technique::Cut(amt) => {
                add =
                    (add + if amt < 0 {
                        deck_size + amt as i128
                    } else {
                        amt as i128
                    }) % deck_size;
            }
            Technique::DealWithIncrement(increment) => {
                let x = multiplicative_inverse(increment as i128, deck_size as i128);
                mul = (mul * x) % deck_size;
                add = (add * x) % deck_size;
            }
        }
    }

    (mul, add)
}

fn modular_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    assert!(modulus > 0 && (modulus - 1) < std::u64::MAX as u128);
    if modulus == 1 {
        return 0;
    }

    let mut res = 1;
    base %= modulus;
    while exp > 0 {
        if (exp % 2) == 1 {
            res = (res * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }

    res
}

fn nr_in_position_after(steps: &[Technique], position: i128, deck_size: i128, rep_count: u64) -> i128 {
    let (mul, add) = get_mul_add_to_reverse_shuffle(steps, deck_size);

    // Explanation:
    // m = multiplier
    // a = addition
    // f(0) = p + 0
    // f(1) = (p) * m + a = pm + a
    // f(2) = (pm + a) * m + a = pm^2 + am + a
    // f(3) = (pm^2 + am + a) * m + a = pm^3 + am^2 + am + a
    // f(4) = (pm^3 + am^2 + am + a) * m + a = pm^4 + am^3 + am^2 + am + a
    //
    // It can also be rewritten as:
    // f(x) = pm^x + g(x)
    // g(0) = 0
    // g(x) = mg(x - 1) + a
    // Where g is a linear non-homogenous recurrence, which can be rewritten as:
    // g(x) = (am^x - a) / (m - 1)
    //
    // Consequently, calculating all repetitions can be done using:
    // f(x) = pm^x + (am^x - a) / (m - 1)

    let mx = modular_pow(mul as u128, rep_count as u128, deck_size as u128) as i128;
    let pmx = (position * mx) % deck_size;
    let amx = (add * mx) % deck_size;
    let inv = multiplicative_inverse(mul - 1, deck_size);
    let res = (pmx + (amx - add) * inv) % deck_size;
    if res < 0 {
        res + deck_size
    } else {
        res
    }
}

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let input = BufReader::new(&input_file);
    let mut steps = Vec::new();
    for line in input.lines() {
        let line = line.unwrap();
        match line.as_ref() {
            "deal into new stack" => {
                steps.push(Technique::DealIntoNewStack);
            },
            x if x.starts_with("cut ") => {
                let cut_value: i64 = x.split("cut ").last().unwrap().parse().unwrap();
                steps.push(Technique::Cut(cut_value));
            },
            x if x.starts_with("deal with increment ") => {
                let increment: i64 = x.split("deal with increment ").last().unwrap().parse().unwrap();
                steps.push(Technique::DealWithIncrement(increment));
            },
            _ => panic!("unexpected input: {}", line)
        }
    }

    // part 1
    const DECK_SIZE: i64 = 10007;
    let mut position: i64 = 2019;
    for step in &steps {
        match step {
            Technique::DealIntoNewStack => position = DECK_SIZE - 1 - position,
            Technique::Cut(cut_value) => position = (position - cut_value) % DECK_SIZE,
            Technique::DealWithIncrement(increment) => position = (position * increment) % DECK_SIZE
        }
    }
    println!("Part 1: {}", position);

    // part 2 solution from https://github.com/Aidiakapi/advent_of_code_2019
    println!("Part 2: {}", nr_in_position_after(&steps, 2020, 119315717514047, 101741582076661));
}