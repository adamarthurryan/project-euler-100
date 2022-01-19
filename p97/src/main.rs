/*
The first known prime found to exceed one million digits was discovered in 1999, and is a Mersenne prime 
of the form 2^6972593−1; it contains exactly 2,098,960 digits. Subsequently other Mersenne primes, of the 
form 2^p−1, have been found which contain more digits.

However, in 2004 there was found a massive non-Mersenne prime which contains 2,357,207 digits: 28433×2^7830457+1.

Find the last ten digits of this prime number.

 */

//convert the exponent to a binary bit string
//use that to compute a fast modular exponentiation (see https://en.wikipedia.org/wiki/Modular_exponentiation)
//multiply and add the remaining terms
//this will overflow u64 but fits u128

#[test]
fn fast_mod_exp_works() {
    assert_eq!(fast_mod_exp(2, 8, 10), 6);
    assert_eq!(fast_mod_exp(2, 8, 100), 56);
    assert_eq!(fast_mod_exp(2, 7, 100), 28);
}

#[test]
fn solves() {
    assert_eq!(solve(), 8739992577);
}


fn main() {
    println!("Solution: {}", solve());
}

fn solve() -> u64 {
    (fast_mod_exp(2, 7830457, 10_000_000_000) * 28433 + 1) % 10_000_000_000
}

//a^k mod m
fn fast_mod_exp(a: u64, k: u64, m: u64) -> u64{
    //represent k as a bit string
    let b = to_bit_string(k);

    let a = a as u128;
    let m = m as u128;
    let mut pow = a % m; //: BigUint = FromPrimitive::from_usize(a % m).unwrap();
    let mut x = 1; // BigUint = FromPrimitive::from_usize(1).unwrap();
    
    for bi in b {
        if bi {
            x = x*pow % m;
        }
        pow = (pow*pow) % m;
    }

    x as u64
}

//returns a bit string representing n, little endian order
fn to_bit_string(n: u64) -> Vec<bool> {
    let mut b = Vec::new();
    let mut n = n;
    while n>0 {
        b.push(n%2 == 1);
        n /= 2;
    }
    
    b
}