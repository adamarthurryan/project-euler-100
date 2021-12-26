/*  Euler's Totient function, φ(n) [sometimes called the phi function], is used to determine the number of 
numbers less than n which are relatively prime to n. For example, as 1, 2, 4, 5, 7, and 8, are all less 
than nine and relatively prime to nine, φ(9)=6.

It can be seen that n=6 produces a maximum n/φ(n) for n ≤ 10.

Find the value of n ≤ 1,000,000 for which n/φ(n) is a maximum.
*/

use primal::Sieve;

#[test]
fn totient_works() {
    let sieve = Sieve::new(1_000);

    assert_eq!(totient(&sieve, 2), 1);
    assert_eq!(totient(&sieve, 3), 2);
    assert_eq!(totient(&sieve, 7), 6);
    assert_eq!(totient(&sieve, 10), 4);
    assert_eq!(totient(&sieve, 93), 60);
    assert_eq!(totient(&sieve, 69), 44);
    assert_eq!(totient(&sieve, 89), 88);
}

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

fn totient(sieve: &Sieve, n:usize) -> usize {
    let factors = sieve.factor(n).unwrap();
    let mut phi: usize = 1;
    for (p,k) in factors {
        phi *= (p.pow((k-1) as u32) as usize) * (p-1)
    }

    return phi;
}