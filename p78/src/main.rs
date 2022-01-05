/*Let p(n) represent the number of different ways in which n coins can be separated into piles. For example, five coins can be separated into piles in exactly seven different ways, so p(5)=7.
Find the least value of n for which p(n) is divisible by one million.
*/

//maybe this would be faster with the Euler transform

use memoize::memoize;

#[test]
fn solves() {
}

fn main() {
    println!("Solution: {}", solve());
}

fn solve() -> usize {
    for n in 2.. {
        let mut sum = 0;
        for k in 0..n {
            sum = (sum + p_mod_million(k as isize, n as isize))% 1_000_000;
        }
        println!("n:{:>10}, sum:{:>10}", n, sum);
        if (sum+1)%1_000_000 == 0 {
            return n;
        }    
    }

    unreachable!();
}

#[memoize]
fn p_mod_million(k:isize, n: isize) ->  usize {
    if k==0 && n==0 { return 1; }
    else if k<=0 || n<=0 { return 0; }

    return (p_mod_million(k, n-k) + p_mod_million(k-1, n-1)) % 1_000_000;
}