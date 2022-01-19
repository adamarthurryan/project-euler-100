/*It is possible to write five as a sum in exactly six different ways:

4 + 1
3 + 2
3 + 1 + 1
2 + 2 + 1
2 + 1 + 1 + 1
1 + 1 + 1 + 1 + 1

How many different ways can one hundred be written as a sum of at least two positive integers?*/

// from WP:
//where p(n) is # partitions of n and pk(n) is # partitions of n with exactly k elements
//p(n) = sum{0<k<=n} pk(n)
//pk(n) = pk(n − k) + pk−1(n − 1)
//so we can use dynamic programming to solve

//another way is with generqting functions
//though it's probably not as fast bc. it requires calculating the sum of divisors 

use memoize::memoize;

#[test]
fn solves() {
    assert_eq!(solve(5),6);
    assert_eq!(solve(100),190569291);
}

fn main () {
    println!("solve(5): {}", solve(5));
    println!("solve(100): {}", solve(100));

}

fn solve (n:usize) -> usize {
    let mut sum = 0;
    for k in 0..n {
        sum+=p(k as isize, n as isize);
    }
    sum
}

#[memoize]
fn p(k:isize, n: isize) ->  usize {
    if k==0 && n==0 { return 1; }
    else if k<=0 || n<=0 { return 0; }

    p(k, n-k) + p(k-1, n-1)
}