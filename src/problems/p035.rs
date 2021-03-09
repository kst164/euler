use crate::primes::PrimeTools;

/*
 * Digits must be 1, 3, 7, 9 only for more than 1 digit
*/

pub fn run() {
    let mut pt = PrimeTools::new();

    let possible_digs = [1, 3, 7, 9];

    // Counting single digit separately (2, 3, 5, 7)
    let mut count = 4;

    for num_digits in 2..=6 {
        // Iterate through all possible nums
        // Just use 4^num_digits as a base 4 number
        for mut i in 0..4usize.pow(num_digits) {
            let mut digits = Vec::with_capacity(num_digits as usize);

            for _ in 0..num_digits {
                let next_digit = possible_digs[i % 4];
                digits.push(next_digit);
                i /= 4;
            }

            if is_circular_prime(&digits, &mut pt) {
                println!("{:?}", get_num(&digits, 0));
                count += 1;
            }
        }
    }

    println!("count: {}", count);
}

fn get_num(digits: &Vec<u64>, offset: usize) -> u64 {
    let mut num = 0;
    for i in (0..digits.len()).rev() {
        num = 10 * num + digits[(i + offset) % digits.len()];
    }
    num
}

fn is_circular_prime(digits: &Vec<u64>, pt: &mut PrimeTools) -> bool {
    // Takes digits if number instead of number itself
    
    // Iterate through all cyclic numbers
    // Offset decides which digit is the first num
    for offset in 0..digits.len() {
        let num = get_num(digits, offset);

        if !pt.is_prime(&num) {
            return false
        }
    }

    true
}
