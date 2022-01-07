use digits::Digits;

#[test]
fn solves() {
    assert_eq!(solve(), 972);
}

fn main() {
    println!("Solution: {}", solve());
}

fn solve() -> usize {
    let mut max_sum = 0;

    for a in 1..100 {
        let mut a_b = Digits::new(a);  
        for _b in 1..100 {
            a_b = a_b * Digits::new(a);
            let sum = a_b.digit_sum();

            if sum>max_sum {
                max_sum = sum;
            }
        }
    }

    max_sum
}
