fn main() {
    let start = 197487;
    let end = 673251;
    let mut valid_count1 = 0;
    let mut valid_count2 = 0;
    for password in start..end {
        if password > 99999 && password < 1000000 {
            let mut double_digits1 = false;
            let mut double_digits2 = false;
            let mut order_valid = true;
            let mut prev_digit = 10;
            let mut prev_prev_digit = 10;
            let mut prev_prev_prev_digit = 10;
            for i in 0..7 {
                let digit = (password / 10_i32.pow(i)) % 10;
                if digit == prev_digit {
                    double_digits1 = true;
                }
                if prev_prev_digit == prev_digit && prev_prev_digit != prev_prev_prev_digit && prev_prev_digit != digit {
                    double_digits2 = true;
                }
                if digit > prev_digit {
                    order_valid = false;
                    break;
                }
                prev_prev_prev_digit = prev_prev_digit;
                prev_prev_digit = prev_digit;
                prev_digit = digit;
            }
            if order_valid && double_digits1 {
                valid_count1 += 1;
            }
            if order_valid && double_digits2 {
                valid_count2 += 1;
                println!("Part 2 pw: {}", password);
            }
        }
    }
    println!("Part 1: {}", valid_count1);
    println!("Part 2: {}", valid_count2);
}