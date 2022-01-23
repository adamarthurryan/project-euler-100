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
use std::collections::HashMap;

#[test]
fn solves() {
    assert_eq!(solve(12), 61);
    assert_eq!(solve(12000), 7587457);
}

fn main() {
    println!("Solution: {}", solve(12000));
    todo!("refactor PrimeFactorsIterator and maybe FactorsIterator into a library crate");

}



fn solve(limit:usize) -> usize {

    //k->n gives the minimum n st. n is a product-sum number of k 
    let mut big_k: HashMap<usize, usize> = HashMap::new();

    let iter = FactorsIterator::new(limit*2).filter(|(_,_, count)| *count>1);

    for (n, sum, count) in iter {

        let size = n - sum + count;

        if (!big_k.contains_key(&size) || *big_k.get(&size).unwrap() > n) && size <= limit {
            big_k.insert(size, n);
        }
    }

    let mut big_k = big_k.into_iter().collect::<Vec<_>>();
    let mut nums = big_k.into_iter().map(|(_,n)| n).collect::<Vec<_>>();
    nums.sort();
    nums.dedup();
    nums.into_iter().sum::<usize>()
}

struct FactorsIterator {
    limit: usize,
    curr: Vec<usize>,
    prod: usize,
    sum: usize,
    count: usize,
    finished: bool
}

impl FactorsIterator {
    fn new(limit: usize) -> Self {
        let curr = vec![0;3*isqrt(limit)];
        FactorsIterator{limit, curr, prod:1, sum:0, count:0, finished:false}
    }
}

impl Iterator for FactorsIterator {
    //product, sum, count
    type Item = (usize,usize,usize);
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished { return None; }

        self.prod *= 2;
        self.sum += 2;
        self.count += 1;

        self.curr[0] += 1;


        let mut j = 0;
        while self.prod > self.limit 
        {    
            self.prod /= (j+2usize).pow(self.curr[j] as u32);
            self.sum -= (j+2)*self.curr[j];
            self.count -= self.curr[j]; 

            self.curr[j] = 0;
            j += 1;
            if j >= self.curr.len() {
                self.finished = true;
                return None;
            }

            self.prod *= j+2;
            self.sum += j+2;
            self.count += 1;
            self.curr[j] += 1;
        }

        Some((self.prod, self.sum, self.count))
    }
}

///////////
 
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

fn isqrt(n: usize) -> usize {
    (n as f64).sqrt() as usize
}