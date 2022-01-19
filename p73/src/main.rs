use fractions::{Fraction, SternBrocotFractions};



#[test]
fn solves() {
    assert_eq!(solve(Fraction::new(1,3), Fraction::new(1,2), 8),3);
    assert_eq!(solve(Fraction::new(1,3), Fraction::new(1,2), 12_000),7295372);
}

fn main() {
    println!("solve(8): {}", solve(Fraction::new(1,3), Fraction::new(1,2), 8));
    println!("Solution: {}", solve(Fraction::new(1,3), Fraction::new(1,2), 12_000));
}

fn solve(a: Fraction, b:Fraction,  max_d: usize) -> usize {
    let sb_fractions = SternBrocotFractions::with_range(|mediant| mediant.d<=max_d, a, b);
    sb_fractions.count()
}