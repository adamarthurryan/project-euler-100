/*Let p(n) represent the number of different ways in which n coins can be separated into piles. For example, five coins can be separated into piles in exactly seven different ways, so p(5)=7.
Find the least value of n for which p(n) is divisible by one million.
*/

//use the Euler tranform, similar to p76, but with streamlined memo
//this should use the pentagonal number expansion

use memoize::memoize;

#[test]
fn p_mod_million_works() {
    //the correct digits of the sequence
    let correct = [1,1,2,3,5,7,11,15,22,30,42,56];
    let mut memo = vec![vec![1;1];1];

    for n in 2..10 {
        memo.push(Vec::with_capacity(n));

        let mut sum = 0;
        for k in 1..=n {
            let p = p_mod_million(k, n, &memo);
            sum += p;
            memo[n-1].push(p);
        }
        assert_eq!(sum, correct[n]);
    }
}

fn main() {
    println!("Solution: {}", solve());
}

fn solve() -> usize {
    let mut memo = vec![vec![1;1];1];

    for n in 2.. {
        memo.push(Vec::with_capacity(n));

        let mut sum = 0;
        for k in 1..=n {
            let p = p_mod_million(k, n, &memo);
            memo[n-1].push(p);
            sum += p;
            if sum>1_000_000 {sum -= 1_000_000; }
        }

        if n%100 == 0 {
            println!("n:{:>10}, sum:{:>10}", n, sum);            
        }
        if sum%1_000_000 == 0 {
            return n;
        }    
    }

    unreachable!();
}

fn p_mod_million(k:usize, n: usize, memo: &Vec<Vec<usize>>) ->  usize {
    //the result is correct for n+1... not for n...
    if k==1 { return 1; }
    else if k<=0 || n<=0 { return 0; }

    let a = if n-k>=k {memo[n-k-1][k-1]} else {0};
    let b = if n>1 && k>1 {memo[n-1-1][k-1-1]} else {1};

    return if a+b>1_000_000 {a+b-1_000_000} else {a+b};
}
