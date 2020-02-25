// Part One
// It is a six-digit number.
// The value is within the range given in your puzzle input.
// Two adjacent digits are the same (like 22 in 122345).
// Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).

fn i32_to_slice(n: i32) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push((n % 10) as u8);
        n = n / 10;
    }
    digits.push(n as u8);
    digits.reverse();
    digits
}

// 112233 meets these criteria because the digits never decrease
// and all re//peated digits are exactly two digits long.
// 123444 no longer meets the criteria (the repeated 44 is part
// of a larger// group of 444).
// 111122 meets the criteria (even though 1 is repeated more than
// twice, it still contains a double 22).

fn check(r: &[u8]) -> bool {
    let mut last_dig: u8 = r[0];
    let mut count_eq = 1;
    let mut pair = false;

    for index in 1..r.len() {
        if last_dig > r[index] {
            return false;
        }
        if last_dig == r[index] {
            count_eq += 1;
        } else {
            match count_eq {
                // Sequence of 3 | 5 equal digits are unforbidden
                3 | 5 => {
                    return false;
                }
                2 => pair = true,
                _ => (),
            }
            count_eq = 1;
        }
        last_dig = r[index];
    }
    if count_eq >= 2 {
        return true;
    }
    pair
}

pub fn run() {
    let mut count: usize = 0;
    for n in 206938..679128 {
        let r = i32_to_slice(n);
        let r = r.as_slice();
        if check(&r) {
            count += 1;
        }
    }
    println!("N Combs: {}", count);
}
