#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seive_works() {
        let table = seive(100);
        assert_eq!(table[2], true);
        assert_eq!(table[3], true);
        assert_eq!(table[4], false);
        assert_eq!(table[5], true);
        assert_eq!(table[6], false);
        assert_eq!(table[7], true);
        assert_eq!(table[8], false);
        assert_eq!(table[9], false);
    }
    #[test]
    fn is_prime_works() {
        let primes = primes(seive(1_000_000));
        assert_eq!(is_prime(&primes,9791), true);
        assert_eq!(is_prime(&primes,6971), true);
        assert_eq!(is_prime(&primes,6973), false);
        assert_eq!(is_prime(&primes,2), true);
        assert_eq!(is_prime(&primes,6), false);

    }

}

//classic Seive of Erasthemes algorithm
//return table of primality for numbers up to to n 
//this would be better as an object that has methods like is_prime() and prime_iter()

pub fn seive(n: u64) -> Vec<bool> {
    let mut a = vec![true; n as usize + 1];
    
    a[0] = false;
    a[1] = false;

    //flag non-primes 
    for i in 2..=isqrt(n) {
        if a[i as usize] {
            for j in (i*i..).step_by(i as usize).take_while(|x| x <= &n) {
                a[j as usize] = false; 
            }
        }
    }
    
    return a;
}

pub fn primes(a: Vec<bool>) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    //gather the primes
    for i in 2..a.len() {
        if a[i] {
            primes.push(i as u64);
        }
    }
    
    return primes;
}

pub fn is_prime(primes: &Vec<u64>, n: u64) -> bool {
    let sqrt_n = isqrt(n);
    for p in primes.iter().take_while(|p| **p <= sqrt_n+1) {
        if n % p == 0 { return false; }
    }
    return true;
} 

//simple integer square root
fn isqrt(n: u64) -> u64 {
    (n as f64).sqrt() as u64
}
