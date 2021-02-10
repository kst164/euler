use crate::primes;

pub fn run() {
    let mut iterator = primes::PrimeIterator::new();

    let mut sum: u64 = 0;
    let mut p: u64 = iterator.get_next();

    while p < 2000000 {
        sum += p;
        p = iterator.get_next();
    }

    println!("{}", sum);
}
