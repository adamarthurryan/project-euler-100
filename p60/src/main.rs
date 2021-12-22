/*The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes and 
concatenating them in any order the result will always be prime. For example, 
taking 7 and 109, both 7109 and 1097 are prime. The sum of these four primes, 792, 
represents the lowest sum for a set of four primes with this property.

Find the lowest sum for a set of five primes for which any two primes concatenate 
to produce another prime.*/

use primes;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concatenate_works() {
        assert_eq!(concatenate(55,68), 5568);
        assert_eq!(concatenate(3, 123091), 3123091);
        assert_eq!(concatenate(213,8), 2138);
    }
}


fn main() {

    let seive = primes::Seive::new(1_000);
    let primes: Vec<u64> = seive.primes().collect();

    //test all pairs in primes?
    //then look for overlaps

    //find all pairs of primes
    let pairs: Vec<_> = Pairs::new(&primes).collect();

    //filter for both composites are prime
    let pairs: Vec<_> = pairs.into_iter().filter(|(&a,&b)| seive.is_prime(concatenate(a,b)) && seive.is_prime(concatenate(b,a))).collect();
    
    //now we are in graph theory
    //the pairs are edges, we need to also build a set of vertices (the primes)
    println!("{:?}", pairs);

    todo!("filter for both composites are prime");

    //find groups...
    unimplemented!();

}

//concatenate the digits of b after the digits of a
fn concatenate(a:u64, b:u64) -> u64 {
    let exp = (b as f64).log10().ceil() as u32;
    return a*10u64.pow(exp)+b;
}

struct Pairs <'a, T> {
    next: (usize,usize),
    source: &'a [T],
}

impl <'a, T> Pairs <'a, T> {
    fn new(source:&[T]) -> Pairs <T> {
        return Pairs{next:(0,1), source};
    }
}

impl <'a, T> Iterator for Pairs <'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.0 >= self.source.len() || self.next.1 >= self.source.len() {
            return None;
        } 

        let item = (&self.source[self.next.0], &self.source[self.next.1]);
        self.next.1 += 1;
        if self.next.1 >= self.source.len() {
            self.next.0 += 1;
            self.next.1 = self.next.0 + 1;
        }

        return Some(item);
    }
}