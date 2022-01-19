/*A natural number, N, that can be written as the sum and product of a given set of at least two natural numbers, {a1, a2, ... , ak} is called a product-sum number: N = a1 + a2 + ... + ak = a1 × a2 × ... × ak.

For example, 6 = 1 + 2 + 3 = 1 × 2 × 3.

For a given set of size, k, we shall call the smallest N with this property a minimal product-sum number. The minimal product-sum numbers for sets of size, k = 2, 3, 4, 5, and 6 are as follows.

k=2: 4 = 2 × 2 = 2 + 2
k=3: 6 = 1 × 2 × 3 = 1 + 2 + 3
k=4: 8 = 1 × 1 × 2 × 4 = 1 + 1 + 2 + 4
k=5: 8 = 1 × 1 × 2 × 2 × 2 = 1 + 1 + 2 + 2 + 2
k=6: 12 = 1 × 1 × 1 × 1 × 2 × 6 = 1 + 1 + 1 + 1 + 2 + 6

Hence for 2≤k≤6, the sum of all the minimal product-sum numbers is 4+6+8+12 = 30; note that 8 is only counted once in the sum.

In fact, as the complete set of minimal product-sum numbers for 2≤k≤12 is {4, 6, 8, 12, 15, 16}, the sum is 61.

What is the sum of all the minimal product-sum numbers for 2≤k≤12000?

 */


use primal::Sieve;
use std::{collections::HashMap, ops::RangeBounds};
fn main() {

    //k->n gives the minimum n st. n is a product-sum number of k 
    let mut big_k: HashMap<usize, usize> = HashMap::new();

    let iter = PrimeFactorsIterator::new(100);
    let primes = iter.primes();
    println!("primes: {:?}", primes);

    for k in iter {

        let n = product_prime_factors(&k, &primes);
        let sum = sum_prime_factors(&k, &primes);
        let non_zero_k = k.iter().filter(|&&ki| ki>0);
        let size = n-sum + non_zero_k.count();
        println!("n: {}, sum: {}, size: {}, {:?}", n, sum, size, k);

        if big_k.contains_key(&size) && *big_k.get(&size).unwrap() > n {
            big_k.insert(size, n);
        }
    }

    println!("{:?}", big_k);

}

fn product_prime_factors(k: &[usize], primes: &[usize]) -> usize {
    let mut n = 1;
    for i in 0..k.len() {
        n *= primes[i].pow(k[i] as u32);
    }

    n
}

fn sum_prime_factors(k: &[usize], primes: &[usize]) -> usize {
    let mut n = 0;
    for i in 0..k.len() {
        if k[i] > 0 {
            n += primes[i].pow(k[i] as u32);
        }
    }

    n
}


struct PrimeFactorsIterator {
    limit: usize,
    curr: Vec<usize>,
    primes: Vec<usize>,
    finished: bool
}

impl PrimeFactorsIterator {
    fn new(limit: usize) -> Self {
        let sqrt_limit = isqrt(limit);
        let sieve = Sieve::new(sqrt_limit);
        let primes = sieve.primes_from(2).collect();
        let curr= vec![0;sieve.prime_pi(sqrt_limit)];

        PrimeFactorsIterator{limit, curr, primes, finished:false}
    } 

    fn primes(&self) -> Vec<usize> {
        self.primes.clone()
    }

}

impl Iterator for PrimeFactorsIterator {
    type Item = Vec<usize>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished { return None; }

        self.curr[0] += 1;

        let mut j = 0;
        while product_prime_factors(&self.curr, &self.primes) > self.limit {
            self.curr[j] = 0;
            j += 1;
            if j >= self.curr.len() {
                self.finished = true;
                return None;
            }
            self.curr[j] += 1
        }
        
        Some(self.curr.clone())
    }
}


fn isqrt(n: usize) -> usize {
    (n as f64).sqrt() as usize
}