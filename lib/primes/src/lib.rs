#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_works() {
        let seive = Seive::new(10);
        assert_eq!(seive.is_prime(2), true);
        assert_eq!(seive.is_prime(3), true);
        assert_eq!(seive.is_prime(37), true);
        assert_eq!(seive.is_prime(64), false);
        assert_eq!(seive.is_prime(89), true);
    }
    #[test]
    fn primes_works() {
        //..
    }
}

struct Seive {
    primality: Vec<bool>,
    limit: u64
}

//classic Seive of Erasthemes algorithm
//return table of primality for numbers up to to n 
pub impl Seive {
    pub fn new(limit: u64) -> Seive {
        let mut primality = vec![true; limit as usize + 1];
        
        primality[0] = false;
        primality[1] = false;

        //flag non-primes 
        for i in 2..=isqrt(limit) {
            if primality[i as usize] {
                for j in (i*i..).step_by(i as usize).take_while(|x| x <= &limit) {
                    primality[j as usize] = false; 
                }
            }
        }
        
        return Seive{primality, limit};
    }

    pub fn primes(&self) -> SeivePrimes {
        SeivePrimes::new(self)
    }

    //returns true if n is a prime
    //panics if n > limit^2
    pub fn is_prime(&self, n:u64) -> bool {
        //lookup values of n that are covered by the seive
        if n <= self.limit {
            return self.primality[n as usize];
        }

        //other values need to be tested
        //fail is n > limit^2
        let sqrt_n = isqrt(n);
        if sqrt_n > self.limit {
            panic!("testing n={} is out of range of seive with limit={}", n, self.limit);
        }

        //get an iterator of primes
        let primes = self.primes();

        //look for a prime factor of n
        for p in primes.take_while(|p| *p <= sqrt_n+1) {
            if n % p == 0 { return false; }
        }
        return true;    
    }
}

struct SeivePrimes<'a> {
    seive: &'a Seive,
    next: u64
}

impl <'a> SeivePrimes<'a> {
    pub fn new (seive: &Seive) -> SeivePrimes {
        SeivePrimes { seive, next:2}
    }
}

impl <'a>Iterator for SeivePrimes<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        println!("self.next: {}", self.next);
        for i in self.next..self.seive.limit+1 {
            if self.seive.primality[i as usize] {
                self.next = i+1;
                return Some(i);
            }
        }

        self.next = self.seive.limit+1;
        return None;
    }
}

//simple integer square root
fn isqrt(n: u64) -> u64 {
    (n as f64).sqrt() as u64
}
