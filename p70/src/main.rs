/*Euler's Totient function, φ(n) [sometimes called the phi function], is used to determine the number of positive numbers less than or equal to n which are relatively prime to n. For example, as 1, 2, 4, 5, 7, and 8, are all less than nine and relatively prime to nine, φ(9)=6.
The number 1 is considered to be relatively prime to every positive number, so φ(1)=1.

Interestingly, φ(87109)=79180, and it can be seen that 87109 is a permutation of 79180.

Find the value of n, 1 < n < 10^7, for which φ(n) is a permutation of n and the ratio n/φ(n) produces a minimum.
*/

use primes::TotientSieve;
use digits::digit_freq;

#[test]
fn solves() {
    assert_eq!(solve(), 8319823);
}
#[test]
fn is_permutation_works() {
    assert!(is_permutation(87109, 79180));
    assert!(!is_permutation(87108, 79180));
}


fn main() {
    println!("Solution: {}", solve());
}

fn solve() -> usize {
    println!("Generating sieve");
    let sieve = TotientSieve::new(10_000_000);
    
    let mut the_min = 10_000_000f64;
    let mut the_n = 0;

    println!("Searching values");
    for n in 2..10_000_000 {
        let phi = sieve.totient(n);
        
        //is it a permutation?
        if is_permutation(n,phi) {
            let ratio = n as f64 / phi as f64;
            if ratio < the_min {
                the_min = ratio;
                the_n = n;
            }
        }
    }

    the_n
}

/*
fn is_permutation(a:usize, b:usize) -> bool {
    let mut a = Digits::new(a);
    a.digits_mut().sort();
    let mut b = Digits::new(b);
    b.digits_mut().sort();

    return a == b;
}
*/

fn is_permutation(a:usize, b:usize) -> bool {
    digit_freq(a) == digit_freq(b)
}

