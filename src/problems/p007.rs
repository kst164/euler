use crate::primes;

pub fn run() {
    let mut prime_things = primes::PrimeIterator::new();
    println!("{}", prime_things.get_prime(&10000usize));
}
