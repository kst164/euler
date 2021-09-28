use crate::primes;

pub fn run() {
    let args: Vec<String> = std::env::args().collect();

    let mut input: u64 = 600851475143;
    if args.len() == 2 {
        input = args[1].parse().unwrap();
        println!("Finding prime factors of {}", input);
    } else {
        println!("No input given, defaulting to {}", input);
    }

    factorize(input);
}

fn factorize(n: u64) {
    let mut prime_iterator = primes::PrimeIterator::new();
    let mut remaining_num: u64 = n;
    let mut sqrt_remaining: u64 = (n as f64).sqrt().floor() as u64;

    let mut next_prime = prime_iterator.get_next();

    while next_prime <= sqrt_remaining {
        if remaining_num % next_prime == 0 {
            while remaining_num % next_prime == 0 {
                remaining_num /= next_prime;
                println!("Factor {}", next_prime);
                /*if remaining_num > 1 {
                    println!("    Remaining {}", remaining_num);
                }*/
            }
            sqrt_remaining = (remaining_num as f64).sqrt().floor() as u64;
        }

        next_prime = prime_iterator.get_next();
    }
    if remaining_num > 1 {
        println!("Factor {}", remaining_num);
    }
}

// Original implementation, before primes lib

/*pub fn run1() {
    let args: Vec<String> = std::env::args().collect();

    let mut input: i64 = 600851475143;
    if args.len() == 2 {
        input = args[1].parse().unwrap();
        println!("Finding prime factors of {}", input);
    } else {
        println!("No input given, defaulting to {}", input);
    }

    let mut primes: Vec<i64> = vec![2];

    let mut remaining_num: i64 = input;
    let mut sqrt_remaining: i64 = (input as f64).sqrt().floor() as i64;
    let mut n: i64 = 2; // Incrementing variable

    while n <= sqrt_remaining {
        if is_prime(n, &primes) {
            primes.push(n);

            if remaining_num % n == 0 {
                while remaining_num % n == 0 {
                    remaining_num /= n;
                    println!("Factor {}", n);
                    println!("    Remaining {}", remaining_num);
                }
                sqrt_remaining = (remaining_num as f64).sqrt().floor() as i64;
            }

            [>if remaining_num % n == 0 {
                println!("Factor {}", n);
            }<]
        }

        n += 1;
    }

}

fn is_prime(n: i64, primes: &Vec<i64>) -> bool {
    if n == 2 {
        return true;
    }
    let sqrt_n: i64 = (n as f64).sqrt().floor() as i64;
    //println!("Checking if {} is prime using {:?}", n, *primes);
    //println!("sqrt_n: {}", sqrt_n);

    for prime in primes {
        if n % prime == 0 {
            return false;
        }
        if *prime >= sqrt_n {
            return true;
        }
    }

    return true;
}*/
