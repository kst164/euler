pub struct PrimeIterator {
    primes: Vec<u64>,
    iterator_pos: usize
}

impl PrimeIterator {
    pub fn new() -> PrimeIterator {
        PrimeIterator {
            primes: vec![2, 3],
            iterator_pos: 0
        }
    }

    pub fn get_next(&mut self) -> u64 {
        self.iterator_pos += 1;

        self.get_prime(&(self.iterator_pos - 1))
    }

    pub fn get_prime(&mut self, n: &usize) -> u64 {
        while self.primes.len() <= *n{
            self.add_prime();
        }

        self.primes[*n]
    }

    fn is_prime(&self, n: &u64) -> bool {
        for p in self.primes.iter() {
            if n % p == 0 {
                return false;
            }
        }

        true
    }

    fn add_prime(&mut self) {
        let mut next_prime: u64 = 2;

        // If there are any elements, get the last one
        if let Some(&largest_prime) = self.primes.last() {
            // Iterate through odd numbers

            next_prime = largest_prime + 2;

            while !self.is_prime(&next_prime) {
                next_prime += 2;
            }
        }

        self.primes.push(next_prime);
    }
}
