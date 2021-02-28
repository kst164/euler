// Math I used: Won't go beyond 999999 (by checking equality of 10^n - 1 = n*9^5)

pub fn run() {
    let mut sum = 0;
    for n in 2..10i32.pow(6) {
        if digit_power_sum(n) == n {
            println!("{}", n);
            sum += n;
        }
    }
    println!("sum: {}", sum);
}

fn digit_power_sum(n: i32) -> i32 {
    let mut num = n;

    let mut sum = 0;

    while num > 0 {
        let dig = num % 10;
        sum += dig.pow(5);
        num /= 10;
    }

    sum
}
