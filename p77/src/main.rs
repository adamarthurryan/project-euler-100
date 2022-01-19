/*It is possible to write ten as the sum of primes in exactly five different ways:

7 + 3
5 + 5
5 + 3 + 2
3 + 3 + 2 + 2
2 + 2 + 2 + 2 + 2

What is the first value which can be written as the sum of primes in over five thousand different ways?*/

//use the Euler transform to solve
//from https://mathworld.wolfram.com/EulerTransform.html
//b(n) = 1/n [cn + sum(k in 1..n-1)[ck*b(n-k)]]
//where cn = sum(p|n and p prime)p  (ie. the sopf - sum of prime factors function)

use memoize::memoize;
use primal::Sieve;
use once_cell::sync::OnceCell;

#[test]
fn solves() {
    assert_eq!(solve(5000),71);
}

fn main() {
    println!("Solution: {}", solve(5000));
}

static SIEVE: OnceCell<Sieve> = OnceCell::new();

fn solve(m: usize) -> usize {
    let sieve_limit = 10_000; 
    let sieve = Sieve::new(sieve_limit);
    SIEVE.set(sieve).unwrap();

    //find first n st b(n) > m
    for n in 2.. {
        //if the number is too big for the sieve to factorize, panic
        if n > sieve_limit.pow(2)  {
            panic!("Exceeded sieve size")
        }
        //search for the large value of b(n)
        if b(n) > m {
            return n;
        }
    }

    unreachable!();
}

#[memoize]
fn b(n: usize) -> usize {
    (
        sopf(n) 
        + (1..(n-1)).map(|k| sopf(k)*b(n-k)).sum::<usize>()
    )/n
}

#[memoize]
fn sopf(n: usize) -> usize {
    let sieve = SIEVE.get().unwrap();
    sieve.factor(n).unwrap().into_iter().map(|(p,_k)| p).sum()
}
