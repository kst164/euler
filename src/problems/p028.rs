pub fn run() {
    let mut sum = 1;

    // Square of side length 2*k + 1
    const K: i32 = 500;

    for n in (1..=(2 * K - 1)).step_by(2) {
        sum += 4 * n * n + 10 * (n + 1);
    }

    println!("{}", sum);
}
