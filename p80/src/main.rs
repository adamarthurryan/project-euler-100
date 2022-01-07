use num::FromPrimitive;
use num::bigint::BigInt;
use num::rational::{Ratio, BigRational};
use digits::Digits;

#[test]
fn sqrt_works() {
    assert_eq!(to_digits(approx_sqrt(2, 5), 66).to_string(), "141421356237309504880168872420969807856967187537694807317667973799");
    assert_eq!(to_digits(approx_sqrt(2, 6), 100).digit_sum(), 475);
}

fn solves() {
    assert_eq(solve(), 40886);
}

fn main() {
    println!("solution: {}", solve());
}

fn is_square(n:usize) -> bool {
    if n>100 {
        unimplemented!("implementation only valid for n<=100");
    }

    //this is pretty lazy
    n==0 || n==1 || n==4 || n==9 || n==16 || n==25 || n==36 || n==49 || n==64 || n==81 || n==100
}

fn solve() -> usize{
    let mut sum = 0;
    for n in 0..=99 {
        if !is_square(n) {
            sum += to_digits(approx_sqrt(n, 8), 100).digit_sum();
        }
    }

    return sum;
}

//Babylonian method for computing square roots
//from https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method 
fn approx_sqrt(n: usize, iterations: usize) -> BigRational {
    //starting approximation for n = a*10^2m = (a/10 +1.2)*10^m for 0<a<100
    //let n=FromPrimitive::number;
    let m = (((n as f32).log10().floor()) as u32) / 2;
    let a = if m>0 { n / 10usize.pow(2*m-2) } else { n };
    let start = ((a as f64)/10.0 + 1.2) * 10.0f64.powf(m as f64);
    
    //convert to bigint ratios
    let start: Ratio<BigInt> = Ratio::from_f64(start).unwrap();
    let n: Ratio<BigInt> = Ratio::from_u64(n as u64).unwrap();

    let mut approx = start;

    //x(i+1) = (xi + n/xi) / 2
    for _ in 0..iterations {
        approx = (&approx + (&n / &approx)) /
            Ratio::from_integer(FromPrimitive::from_u64(2).unwrap());
    }

    approx
}

fn to_digits(rational: BigRational, num_digits: usize) -> Digits {
    let n=rational.numer();
    let d=rational.denom();

    let scale_plus_2 = format!("1{}", "0".repeat(num_digits+2)).parse::<BigInt>().unwrap();

    let string = (n*scale_plus_2/d).to_str_radix(10);
    let digits = string[0..num_digits].chars().rev().map(|c| c.to_digit(10).unwrap() as usize);
    Digits::from_digits(&digits.collect::<Vec<usize>>())
}

//for root of n: 
//initial approximation: x0
//xi+1 = (xi + n/xi)/2

//xi is a bigint?
//with mantissa