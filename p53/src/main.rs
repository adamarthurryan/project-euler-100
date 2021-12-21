//count the number of times nCk > 1000000 for n<=100

use std::collections::HashMap;
use std::cmp::min;

fn main() {
    let limit = 1_000_000;
    let mut cache: HashMap<(u64,u64), u64> = HashMap::new();
    for n in 0..100 {
        let k = n/2;
        println!("n:{}, k:{}, nCk: {}", n, k, limited_binomial_function(&mut cache, n, k, limit));
    }

    let mut count = 0;
    for n in 0..=100 {
        for k in 0..=n {
            if limited_binomial_function(&mut cache, n, k, limit) == limit {
                count += 1;
            }
        }
    }

    println!("answer: {}", count);
}


//for n, k, and limit, returns nCk up to limit
fn limited_binomial_function(cache:&mut HashMap<(u64,u64), u64>, n:u64, k:u64, limit:u64) -> u64 {
    //test the cache
    if let Some(&val) = cache.get(&(n,k)) {
        return val;
    }

    if n==0 || k==0 || n==k {
        return 1;
    }

    let val = limited_binomial_function(cache, n-1, k-1, limit) 
        + limited_binomial_function(cache, n-1, k, limit);
    
    let val = min(val, limit);

    cache.insert((n,k), val);
    return val;
}