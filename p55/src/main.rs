/*
If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.
A number that never forms a palindrome through the reverse and add process is called a Lychrel number.
Count the Lychrel numbers below 10000.

No number below 10000 has ever been shown to become a palindrome after _more_ than 50 iterations.
So, we'll only test numbers to 50 iterations.
*/

use digits;

fn main() {
    let mut count_lychrel = 0;

    for n in 0..10_000 {
        if is_lychrel(n) {
            count_lychrel+=1;
            println!("{} is lychrel!", n);
        }
    }

    println!("number of lychrel numbers below 10000: {}", count_lychrel);
}

fn is_lychrel(n:u64) -> bool {
    let mut n = digits::to_digits(n);

    for i in 0..50 {
        n = reverse_add(&n);
        if digits::is_palindrome(&n) {
            return false;
        }
    }
    return true;
}

//reverse and add the two numbers in base 10 digit representation
fn reverse_add(n:&Vec<u64>) -> Vec<u64> {
    let pairs = n.iter().zip(n.iter().rev());
    let mut rem = 0;
    let mut sum: Vec<u64> = Vec::new();


    for (&ai, &bi) in pairs {
        let ci = ai+bi+rem;
        let val = ci%10;
        rem = ci/10;
        sum.push(val);
    }

    while rem>0 {
        sum.push(rem % 10);
        rem /= 10;
    }

    return sum;
}

