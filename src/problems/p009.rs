pub fn run() {
    for a in 1..293 {
        for c in 414..500 {
            let b = 1000 - (a + c);
            if a * a + b * b == c * c {
                println!("{:3} + {:3} + {:3} = 1000", a, b, c);
            }
        }
    }
}
