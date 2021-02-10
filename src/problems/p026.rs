extern crate num_bigint;
use num_bigint::BigInt;

pub fn run() {
    let mut max_len = 0;
    let mut max_at = 1;

    for d in (1..1000).step_by(2) {
        if d % 5 == 0 {
            continue;
        }
        let len = recur_len(&d);
        if len >= max_len {
            println!("{:3} {:4}", len, d);
            max_len = len;
            max_at = d;
        }
    }

    println!("{}", max_at);
}

fn recur_len(d: &u32) -> u32 {
    let big_d = BigInt::from(*d);
    let ten = BigInt::from(10);
    let one = BigInt::from(1);
    let zero = BigInt::from(0);

    let mut k = 1;

    loop {
        let ten_to_the_k_minus_1 = &ten.pow(k) - &one;

        if &ten_to_the_k_minus_1 % &big_d == zero {
            break;
        } else {
            k += 1;
        }
    }

    k
}
