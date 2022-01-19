use primal::Sieve;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totient_works() {
        let sieve = Sieve::new(1_000);
    
        assert_eq!(totient(&sieve, 2), 1);
        assert_eq!(totient(&sieve, 3), 2);
        assert_eq!(totient(&sieve, 7), 6);
        assert_eq!(totient(&sieve, 10), 4);
        assert_eq!(totient(&sieve, 93), 60);
        assert_eq!(totient(&sieve, 69), 44);
        assert_eq!(totient(&sieve, 89), 88);
    }

    #[test]
    fn totient_sieve_works() {
        let ts = TotientSieve::new(1_000);
    
        assert_eq!(ts.totient(2), 1);
        assert_eq!(ts.totient(3), 2);
        assert_eq!(ts.totient(7), 6);
        assert_eq!(ts.totient(10), 4);
        assert_eq!(ts.totient(93), 60);
        assert_eq!(ts.totient(69), 44);
        assert_eq!(ts.totient(89), 88);
    }
}


pub fn totient(sieve: &Sieve, n:usize) -> usize {
    let factors = sieve.factor(n).unwrap();
    let mut phi: usize = 1;
    for (p,k) in factors {
        phi *= (p.pow((k-1) as u32) as usize) * (p-1)
    }

    phi
}

pub struct TotientSieve {
    limit: usize,
    values: Vec<usize>
}

/* There is a faster algorithm for this:
Now, this is pretty simple but it requires a division at each iteration of step 3. We can instead start from 1 and only have to do multiplication:

1. Initialize the array from 2:n with the value 1, and start at m = 2.
2. If the value at m is still 1, m is prime.
3. Calculate the largest  mk≤n  and set  t[i]=mk−i−1(m−1)  via repeated multiplication. Visit  2mk,3mk,...  and multiply those entries by  t[k] .
4. Do the same for  mk−1  but instead skip the multiples of  m  while iterating. I.e., if m=2 then visit  mk−1,3mk−1,5mk−1,...  and multiply each by  t[k−1] . Repeat for all powers of  m . including  m1 .
5. Increment m and go back to step 2

Step 4 can be done as a double-loop to avoid a division check. (We need to not visit  xmk  and then visit it again as a multiple of  m  or  m2 .)
*/
impl TotientSieve {
    pub fn new(limit: usize) -> Self {
        //start with values 2..limit
        let mut values: Vec<usize> = (2..=limit).collect();
        
        for m in 2..=limit {
            //if m is a prime
            if values[m-2] == m {
                //multiply each km by (1-1/m) ie. v-v/m
                for n in (m..).step_by(m).take_while(|&k| k <= limit) {
                    values[n-2] = values[n-2] - values[n-2]/m;
                }
            }
        }
        
        TotientSieve{limit, values}
    }

    pub fn totient(&self, n: usize) -> usize {
        if n==0 {
            panic!("phi(0) is undefined");
        }
        if n==1 {
            return 1;
        }
        if n-2 > self.limit {
            panic!("Attempted to look up phi({}) for sieve with limit {}", n, self.limit)
        }

        self.values[n-2]
    }
}