/*
If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.
A number that never forms a palindrome through the reverse and add process is called a Lychrel number.
Count the Lychrel numbers below 10000.

No number below 10000 has ever been shown to become a palindrome after _more_ than 50 iterations.
So, we'll only test numbers to 50 iterations.
*/

use digits::Digits;
#[test]
fn solves() {
    assert_eq!(solve(), 249);
}


fn solve() -> usize {
    let mut count_lychrel = 0;

    for n in 0..10_000 {
        if is_lychrel(n) {
            count_lychrel+=1;
        }
    }

    count_lychrel
}

fn main() {
    let count_lychrel = solve();
    println!("number of lychrel numbers below 10000: {}", count_lychrel);
}

fn is_lychrel(n:usize) -> bool {
    let mut n = Digits::new(n);

    for _i in 0..50 {
        //is there a more elegant way to do this?
        let mut n_prime = n.clone();
        n_prime.digits_mut().reverse();
        n = n+n_prime;
        if n.is_palindrome() {
            return false;
        }
    }
    true
}
