/*
 * FUNCTION LIST
 * pub new
 * pub get_prime
 * pub is_prime
 * pub prime_factorization
 * pub factors
 * pub factor_sum
 * pub factor_count
 *
 * add_prime
 * fill_n_primes
 * fill_primes_upto
 * is_next_prime
*/
use std::collections::{BTreeMap, BTreeSet};

pub struct PrimeTools {
    primes: Vec<u64>,
    iterator_pos: usize,
}

pub struct PrimeIterator<'a> {
    pt: &'a mut PrimeTools,
    pos: usize,
}

impl<'a> Iterator for PrimeIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.pos += 1;

        Some(self.pt.get_prime(&(self.pos - 1)))
    }
}

impl<'a> IntoIterator for &'a mut PrimeTools {
    type Item = u64;
    type IntoIter = PrimeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iterator_pos += 1;

        PrimeIterator {
            pt: self,
            pos: 0
        }
    }
}

impl PrimeTools {
    pub fn new() -> PrimeTools {
        PrimeTools {
            primes: vec![2, 3],
            iterator_pos: 0,
        }
    }

    pub fn iter_primes(&mut self) -> PrimeIterator {
        PrimeIterator {
            pt: self,
            pos: 0,
        }
    }

    pub fn get_prime(&mut self, n: &usize) -> u64 {
        // To get index n, need n+1 primes
        self.fill_n_primes(&(*n + 1));

        self.primes[*n]
    }

    pub fn is_prime(&mut self, n: &u64) -> bool {
        // Fill primes till square root of n
        let sqrt_n: u64 = (*n as f64).sqrt().floor() as u64;
        self.fill_primes_upto(&sqrt_n);

        for p in self.primes.iter() {
            if p > &sqrt_n {
                return true;
            }
            if n % p == 0 {
                return false;
            }
        }

        true
    }

    pub fn prime_factorization(&mut self, n: &u64) -> BTreeMap<u64, usize> {
        // Fill primes till square root of n
        let sqrt_n: u64 = (*n as f64).sqrt().floor() as u64;
        self.fill_primes_upto(&sqrt_n);

        let mut remaining_num: u64 = *n;
        let mut sqrt_remaining: u64 = (*n as f64).sqrt().floor() as u64;
        let mut factors = BTreeMap::new();

        for &prime in self.primes.iter() {
            if remaining_num % prime == 0 {
                let mut exp: usize = 0;
                while remaining_num % prime == 0 {
                    remaining_num /= prime;
                    exp += 1;
                }

                sqrt_remaining = (remaining_num as f64).sqrt().floor() as u64;
                factors.insert(prime, exp);
            }
            if prime > sqrt_remaining {
                if remaining_num != 1 {
                    factors.insert(remaining_num, 1);
                }
                break;
            }
        }

        factors
    }

    pub fn factors(&mut self, n: &u64) -> BTreeSet<u64> {
        let prime_factors = self.prime_factorization(n);

        let primes: Vec<u64> = prime_factors.keys().cloned().collect();
        let exps: Vec<usize> = prime_factors.values().cloned().collect();

        let mut current_exps: Vec<usize> = vec![0; prime_factors.len()];

        let mut factors = BTreeSet::new();

        loop {
            factors.insert(get_num(&primes, &current_exps));

            // Iterate current_exps through all possible values, then break
            let mut will_break = true;
            for i in 0..exps.len() {
                if current_exps[i] < exps[i] {
                    current_exps[i] += 1;
                    will_break = false;
                    break;
                } else {
                    current_exps[i] = 0;
                }
            }

            if will_break {
                break;
            }
        }

        factors
    }

    pub fn factor_sum(&mut self, n: &u64) -> u64 {
        let prime_factors = self.prime_factorization(n);
        let mut sum = 1;
        for (&prime, &exp) in prime_factors.iter() {
            sum *= prime.pow(exp as u32 + 1) - 1;
            sum /= prime - 1;
        }
        sum
    }

    pub fn factor_count(&mut self, n: &u64) -> usize {
        let prime_factors = self.prime_factorization(&n);
        let mut num_factors: usize = 1;
        for exp in prime_factors.values() {
            num_factors *= exp + 1;
        }

        num_factors
    }

    fn add_prime(&mut self) -> u64 {
        let mut next_prime: u64 = 2;

        // Get last element
        if let Some(&largest_prime) = self.primes.last() {
            // Iterate through odd numbers

            next_prime = largest_prime + 2;

            while !self.is_next_prime(&next_prime) {
                next_prime += 2;
            }
        }

        self.primes.push(next_prime);

        next_prime
    }

    fn fill_n_primes(&mut self, n: &usize) {
        while self.primes.len() < *n {
            self.add_prime();
        }
    }

    fn fill_primes_upto(&mut self, n: &u64) {
        let largest_prime = self.primes.last().unwrap();
        if largest_prime < n {
            while self.add_prime() <= *n {}
        }
    }

    fn is_next_prime(&mut self, n: &u64) -> bool {
        // Just for add_prime()

        for p in self.primes.iter() {
            if n % p == 0 {
                return false;
            }
        }

        true
    }
}

fn get_num(primes: &Vec<u64>, exps: &Vec<usize>) -> u64 {
    // Converts prime factorisation to a number

    let mut num: u64 = 1;

    for i in 0..primes.len() {
        num *= primes[i].pow(exps[i] as u32);
    }

    num
}
