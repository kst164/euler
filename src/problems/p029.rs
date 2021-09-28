use std::collections::BTreeSet;

// Math I used:
//  Find total entries by powers of 2
//  Find total entries by powers of 3
//  Find total entries by powers of 5, will be same for 6, 7, 10
//  All other numbers that aren't powers of these numbers will give 99 entries each (42, 31, etc)

pub fn run() {
    let p2 = power_count(2);
    let p3 = power_count(3);
    let p5 = power_count(5);

    let ans = p2 + p3 + 4 * p5 + 99 * (99 - 6 - 4 - 4 * 2); // 6 for 2^b, 4 for 3^b, 2 each for {5, 6, 7, 10}^b

    println!("{}", ans);
}

fn power_count(a: i32) -> usize {
    let mut powers = BTreeSet::new();

    let mut base_power = 1;

    while a.pow(base_power) <= 100 {
        for exp in 2..=100 {
            powers.insert(base_power * exp);
        }
        base_power += 1;
    }

    powers.len()
}
