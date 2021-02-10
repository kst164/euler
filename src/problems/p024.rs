pub fn run() {
    /*let mut n = 1000000;
    let mut arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut i = 1;

    while i < 10 {
        for i in (0..10).rev() {
            if i == 0 { return; }
            // Gave up here
        }
    }*/

    // Did this much, now figure out rest on your own
    let mut n = 1000000;
    print!("{} = ", n);
    for i in (1..10).rev() {
        let i_fac = factorial(&i);
        print!("{}*{}! + ", n / i_fac, i);
        n %= i_fac;
    }
    println!("");
}

fn factorial(n: &u64) -> u64 {
    let mut ans = 1;
    for i in 1..=*n {
        ans *= i;
    }

    ans
}
