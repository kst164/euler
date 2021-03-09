/*
 * Only possiblities:
 * 2dig * 3dig = 4dig
 * 1dig * 4dig = 4dig
*/

use std::collections::BTreeSet;

pub fn run() {
    let mut products = BTreeSet::new();

    // 4 = 2 * 3
    for a in 10..99 {
        // If a is divisible by 5, skip
        if a % 5 == 0 {
            continue;
        }

        for b in 100..999 {
            // If b is divisible by 5, skip
            if b % 5 == 0 {
                continue;
            }

            let prod = a * b;

            // product must have 4 digits
            if prod < 1000 || prod > 9999 {
                continue;
            }

            if is_pan_digital_set(&mut [a, b, prod]) {
                println!("{:2} * {:4} = {:4}", a, b, prod);
                products.insert(prod);
            }
        }
    }

    // 4 = 1 * 4
    for a in 1..9 {
        // If a is 5, skip
        if a == 5 {
            continue;
        }

        for b in 1000..9999 {
            // If b is divisible by 5, skip
            if b % 5 == 0 {
                continue;
            }

            let prod = a * b;

            // product must have 4 digits
            if prod < 1000 || prod > 9999 {
                continue;
            }

            if is_pan_digital_set(&mut [a, b, prod]) {
                println!("{:2} * {:3} = {:4}", a, b, prod);
                products.insert(prod);
            }
        }
    }

    let mut total = 0;

    for prod in products.iter() {
        total += prod;
    }

    println!("{}", total);
}

fn is_pan_digital_set(nums: &mut [usize]) -> bool {
    let mut covered = [false; 9];

    for num in nums.iter_mut() {
        while *num > 0 {
            let last_dig = *num % 10;
            if last_dig == 0 {
                // No zeroes allowed
                return false;
            } else if covered[last_dig - 1] {
                // Repeated digit
                return false;
            } else {
                // All good
                covered[last_dig - 1] = true;
            }

            *num /= 10;
        }
    }

    for dig_covered in covered.iter() {
        if !dig_covered {
            return false;
        }
    }

    true
}
