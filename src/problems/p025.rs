extern crate num_bigint;
use num_bigint::BigInt;

pub fn run() {
    let ten_to_999 = BigInt::from(10).pow(999);

    let mut current = BigInt::from(0);
    let mut next = BigInt::from(1);
    let mut tmp;
    let mut i = 0;

    while current < ten_to_999 {
        tmp = next.clone();
        next += current;
        current = tmp;
        i += 1;
    }
    println!("{}", i);
}
