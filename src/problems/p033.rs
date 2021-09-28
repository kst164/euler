/*
 * (10a + b) / (10b + c) == a / c
 *
 * Simplify,
 *
 * b = 9ac / (10a - c)
*/

struct Fraction(u16, u16);

impl Fraction {
    fn equals(self, other: &Fraction) -> bool {
        self.0 * other.1 == other.0 * self.1
    }
}

pub fn run() {
    for a in 1..9 {
        for c in 1..9 {
            if a == c {
                continue;
            }
            if (9 * a * c) % (10 * a - c) == 0 {
                let b = 9 * a * c / (10 * a - c);

                if b < 10 {
                    println!("{a}{b} / {b}{c} = {a}/{c}", a = a, b = b, c = c);
                }
            }
        }
    }
}
