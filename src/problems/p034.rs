pub fn run() {
    for n in 1.. {
        if factorial_sum(n) == n {
            println!("{}", n);
        }
    }
}

fn factorial(n: u32) -> u32{
    let mut fact = 1;

    for k in 2..=n {
        fact *= k;
    }

    fact
}

fn factorial_sum(mut n: u32) -> u32{
    let mut sum = 0;

    while n > 0 {
        sum += factorial(n % 10);
        n /= 10;
    }

    sum
}
