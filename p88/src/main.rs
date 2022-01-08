
/*

The smallest number expressible as the sum of a prime square, prime cube, and prime fourth power is 28. In fact, there are exactly four numbers below fifty that can be expressed in such a way:

28 = 2^2 + 2^3 + 2^4
33 = 3^2 + 2^3 + 2^4
49 = 5^2 + 2^3 + 2^4
47 = 2^2 + 3^3 + 2^4

How many numbers below fifty million can be expressed as the sum of a prime square, prime cube, and prime fourth power?
*/

use std::collections::HashSet;
use primal::Primes;

#[test]
fn solves() {
    assert_eq!(solve(50), 4);
    assert_eq!(solve(50_000_000), 1097343);
}

fn main() {
    println!("solve(50): {}", solve(50));
    println!("solution: {}", solve(50_000_000));
}

fn solve(n: usize) -> usize {
    let sqrt_n = (n as f64).sqrt().floor() as usize;
    let primes: Vec<usize> = Primes::all().take_while(|&p| p <= sqrt_n).collect();

    let mut nums: HashSet<usize> = HashSet::new();

    'i: for i in 0..primes.len() {
        'j: for j in 0..primes.len() {
            'k: for k in 0..primes.len() {
                
                let x = primes[i].pow(4)+primes[j].pow(3)+primes[k].pow(2);
                
//                println!("{}^4 + {}^3 + {}^2 = {}", primes[i], primes[j], primes[k], x);
                if x < n {
                    nums.insert(x);
                }
                else {
                    if j>1 && k==1 {
                        break 'j;
                    }
                    else if k==1 && j==1 {
                        break 'i;
                    }
                    else {
                        break 'k;
                    }
                }
            }
        }
    }

    return nums.len();
}
