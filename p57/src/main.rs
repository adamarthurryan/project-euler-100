//take the continued fraction for root(2): [1;2,2,...]
//expand it into the series of convergents pi/qi
//count the number of times pi has more digits than qi for 0<=i<1000

//used https://crypto.stanford.edu/pbc/notes/contfrac/ as a source

use num::bigint::BigInt;
use num::{Zero, One};


#[test]
fn solves() {
    assert_eq!(solve(8), 1);
    assert_eq!(solve(1000), 153);
}

fn main() {
    println!("solution: {}", solve(1000));
}

//generate the first n convergents of sqrt(2) 
//and count how many have more digits in the numerator than the denominator
fn solve (n:usize) -> usize {

    let mut p:Vec<BigInt> = vec![BigInt::zero(), BigInt::one(), BigInt::one()];
    let mut q:Vec<BigInt> = vec![BigInt::one(), BigInt::zero(), BigInt::one()];

    let mut count = 0;

    for _i in 0..n {
        //hardcoding the series for root 2 here
        let (pi, qi) = convergent(&p, &q, 2);
        p.push(pi.clone());
        q.push(qi.clone());

        if pi.to_str_radix(10).len() > qi.to_str_radix(10).len() {
            count += 1;
        }
    }

    count
}

//calculates the ith convergent of the continued fraction with the current ai coefficient given
//uses the magic table method from https://crypto.stanford.edu/pbc/notes/contfrac/definition.html
fn convergent(p:&[BigInt], q:&[BigInt], ai: usize) -> (BigInt, BigInt) {
    (
        &p[p.len()-1]*ai+&p[p.len()-2],
        &q[q.len()-1]*ai+&q[q.len()-2]
    )
}
