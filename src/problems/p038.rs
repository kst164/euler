use std::collections::BTreeSet;

static POWERS: &[u64; 11] = &[
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
];

pub fn run() {
    let mut max = 0;

    for n in 1..10000 {
        if let Some(pan_num) = get_pandigital_num(n) {
            if pan_num > max {
                max = pan_num;
            }
        }
    }

    println!("{}", max);
}

// Returns Some(pan_num) if its pandigital, otherwise None
fn get_pandigital_num(n: u64) -> Option<u64> {
    let mut covered_digs = BTreeSet::new();
    let mut pan_num = 0;

    for multiplier in 1.. {
        let const_multiple = n * multiplier;
        let mut multiple = const_multiple;
        let mut multiple_dig_count = 0;

        while multiple > 0 {
            let last_dig = multiple % 10;

            if last_dig == 0 {
                // 0 isn't allowed
                return None;
            } else if !covered_digs.insert(last_dig) {
                // Checking that digit is new (insert returns false if element is already in set)
                return None;
            }

            multiple_dig_count += 1;
            multiple /= 10;
        }

        pan_num = pan_num * POWERS[multiple_dig_count] + const_multiple;

        if covered_digs.len() == 9 {
            return Some(pan_num);
        }
    }
    None
}

// Anything less than 1 is a 0 digit number,
// and anything more than 10 digits is a 11 digit number,
// but that doesn't really matter here
fn digit_count(n: u64) -> usize {
    for i in 0..POWERS.len() {
        if n < POWERS[i] {
            return i;
        }
    }

    11
}
