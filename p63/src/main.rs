/*
The 5-digit number, 16807=7^5, is also a fifth power. Similarly, the 9-digit number, 134217728=8^9, is a ninth power.

How many n-digit positive integers exist which are also an nth power?
*/

use digits::Digits;

#[test]
fn solves() {
    assert_eq!(solve(), 49);
}

fn main() {
    println!("Solution: {}", solve());
}

fn solve() -> usize {
    (1..=9).map(|n: usize| (1..)
            
            .map(|e: usize| (e, Digits::new(n).pow(e)))
            .take_while(|(e,x)| x.len() == *e)
            .map(|(e,x)| (e,x.to_string()))
            .collect::<Vec<_>>()
            .len()
        ).sum()
}


