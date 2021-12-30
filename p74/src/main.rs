/*
The number 145 is well known for the property that the sum of the factorial of its digits is equal to 145:

1! + 4! + 5! = 1 + 24 + 120 = 145

Perhaps less well known is 169, in that it produces the longest chain of numbers that link back to 169; it turns out that there are only three such loops that exist:

169 → 363601 → 1454 → 169
871 → 45361 → 871
872 → 45362 → 872

It is not difficult to prove that EVERY starting number will eventually get stuck in a loop. For example,

69 → 363600 → 1454 → 169 → 363601 (→ 1454)
78 → 45360 → 871 → 45361 (→ 871)
540 → 145 (→ 145)

Starting with 69 produces a chain of five non-repeating terms, but the longest non-repeating chain with a starting number below one million is sixty terms.

How many chains, with a starting number below one million, contain exactly sixty non-repeating terms?
*/


use digits::Digits;
use std::collections::{HashSet, HashMap};

#[test]
fn solves() {
    assert_eq!(solve(),402);
}

fn main() {
    println!("Solution: {}", solve());
}

/* This solution is very slow. Some new ideas are needed. Maybe a short list of possible loop ending values? 
Is there a way to not have to test every string?
Memoization of chain lengths seems like the preferred solution in the forums.
*/
fn solve() -> usize {
    //cache the factorials from 0 through 9
    let factorials: Vec<usize> = (0..=9).map(|n| factorial(n)).collect();
    println!("Buckle up, this is a slow one.");
    let mut count = 0;
    for n in 1..1_000_000 {
        if n%10_000 == 0 {
            println!("n: {}, count:{}", n, count);
        }
        if count_steps(n, &factorials, &mut HashSet::new(), &mut HashMap::new()) == 60 {
            count += 1;
        }
    }
    return count;
}

fn count_steps(n: usize, factorials: &Vec<usize>,  hits: &mut HashSet<usize>, memo: &mut HashMap<usize,usize>) -> usize {
    if let Some(&count) = memo.get(&n) {
        return count;
    }
    
    if hits.contains(&n) {
        return 0;
    }
    hits.insert(n);


    let digits = Digits::new(n);
    let n_prime = digits.digits().into_iter().map(|&d| factorials[d]).sum();
    let count = 1 + count_steps(n_prime, factorials, hits, memo);
    memo.insert(n,count);
    return count;
}

fn factorial(n: usize) -> usize {
    if n==0 { 1 }
    else { n*factorial(n-1) }
}



/*
145
871
872
169
*/