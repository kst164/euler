use std::collections::BTreeSet;
use crate::primes::PrimeTools;

pub fn run() {
    let mut abundant_nums = vec![];
    let mut prime_tools = PrimeTools::new();

    const CHECK_TILL: u64 = 28123;

    for n in 1..=CHECK_TILL {
        if prime_tools.factor_sum(&n) > 2 * n {
            abundant_nums.push(n);
        }
    }


    let mut abundant_sum_set = BTreeSet::new();

    for i in 0..abundant_nums.len() {
        for j in i..abundant_nums.len() {
            abundant_sum_set.insert(abundant_nums[i] + abundant_nums[j]);
        }
    }

    let mut sum = 0;

    for i in 1..=CHECK_TILL {
        if !abundant_sum_set.contains(&i) {
            sum += i;
        }
    }

    println!("{}", sum);
}
