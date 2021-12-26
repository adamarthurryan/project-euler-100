/*  Euler's Totient function, φ(n) [sometimes called the phi function], is used to determine the number of 
numbers less than n which are relatively prime to n. For example, as 1, 2, 4, 5, 7, and 8, are all less 
than nine and relatively prime to nine, φ(9)=6.

It can be seen that n=6 produces a maximum n/φ(n) for n ≤ 10.

Find the value of n ≤ 1,000,000 for which n/φ(n) is a maximum.
*/

use primal::Sieve;
use primes::totient;


#[test]
fn solves() {
    assert_eq!(solve(), 510510);
}


fn main() {
    println!("Solution: {}", solve());
}

fn solve() -> usize {
    let sieve = Sieve::new(1_000);

    let mut the_max = 0f64;
    let mut the_n = 1;

    for n in 1..=1_000_000 {
        let phi = totient(&sieve, n);
        let n_over_phi = n as f64/phi as f64;
        if n_over_phi > the_max {
            the_max = n_over_phi;
            the_n = n;
        }
    }

    return the_n;
}

