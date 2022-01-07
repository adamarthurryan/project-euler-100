extern crate num;

use num::FromPrimitive;
use num::bigint::BigInt;
use num::rational::{Ratio, BigRational};

fn main() {
    println!("sqrt(2):", tens(approx_sqrt(2, 8)));
}

//from num docs
fn approx_sqrt(number: u64, iterations: usize) -> BigRational {
    let start: Ratio<BigInt> = Ratio::from_integer(FromPrimitive::from_u64(number).unwrap());
    let mut approx = start.clone();

    for _ in 0..iterations {
        approx = (&approx + (&start / &approx)) /
            Ratio::from_integer(FromPrimitive::from_u64(2).unwrap());
    }

    approx
}

fn 

//for root of n: 
//initial approximation: x0
//xi+1 = (xi + n/xi)/2

//xi is a bigint?
//with mantissa