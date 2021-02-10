use crate::primes::PrimeTools;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let mut thing = Thing::new();

    let mut d_values = HashMap::new();

    for n in 1..10000 {
        d_values.insert(n, thing.d(&n));
    }

    let mut amicable_nums = HashSet::new();

    for n in 1..10000 {
        if let Some(d_n) = d_values.remove(&n) {
            if d_n >= 10000 { continue };

            if let Some(d_d_n) = d_values.remove(&d_n) {
                if n == d_d_n {
                    amicable_nums.insert(n);
                    amicable_nums.insert(d_n);
                }
            }
        }

    }

    println!("{:?}", amicable_nums);

    let mut sum = 0;

    for num in amicable_nums.iter() {
        sum += num;
    }
    println!("{}", sum);
}

struct Thing {
    prime_tools: PrimeTools
}

impl Thing {
    fn new() -> Thing {
        Thing {
            prime_tools: PrimeTools::new()
        }
    }

    fn d(&mut self, n: &u64) -> u64 {
        let factors = self.prime_tools.prime_factorization(&n);

        let mut d = 1;

        for (&prime, &exp) in factors.iter() {
            d *= prime.pow(exp as u32 + 1) - 1;
            d /= prime - 1;
        }

        d - n
    }

    // First approach. Also works, but iterates through every factor, so slower
    fn d1(&mut self, n: &u64) -> u64 {
        if *n == 1 { return 0 };
        let factors = self.prime_tools.prime_factorization(&n);

        let primes: Vec<u64> = factors.keys().cloned().collect();
        let exps: Vec<usize> = factors.values().cloned().collect();

        // current_exps is the factorization of the current number
        let mut current_exps: Vec<usize> = vec![0; factors.len()];
        let mut sum: u64 = 0;

        loop {
            sum += get_num(&primes, &current_exps);

            // Iterate the loop
            // Iterates current_exps through all factors of n
            for i in 0..exps.len() {
                if current_exps[i] < exps[i] {
                    current_exps[i] += 1;
                    break;
                } else {
                    if i < exps.len() - 1 {
                        current_exps[i] = 0;
                    } else {
                        // sum - n because proper divisors
                        return sum - n;
                    }
                }
            }
        }
    }
}

fn get_num(primes: &Vec<u64>, exps: &Vec<usize>) -> u64{
    let mut num: u64 = 1;

    for i in 0..primes.len() {
        num *= primes[i].pow(exps[i] as u32);
    }

    num
}

/*fn d(n: &u64) -> u64 {
    let factors = primes::PrimeTools::prime_factorization(&n);

    let primes = factors.keys().collect();
    let exps = factors.values().collect();

    println!("{:?}", primes);
    println!("{:?}", exps);

    // Iterating "exponents" through all possible factorizations of a factor of n
    let mut exponents: Vec<usize> = vec![0; factors.len()];

    1
}*/
