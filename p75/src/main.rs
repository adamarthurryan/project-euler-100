/*
It turns out that 12 cm is the smallest length of wire that can be bent to form an integer sided right angle triangle in exactly one way, but there are many more examples.

12 cm: (3,4,5)
24 cm: (6,8,10)
30 cm: (5,12,13)
36 cm: (9,12,15)
40 cm: (8,15,17)
48 cm: (12,16,20)

In contrast, some lengths of wire, like 20 cm, cannot be bent to form an integer sided right angle triangle, and other lengths allow more than one solution to be found; for example, using 120 cm it is possible to form exactly three different integer sided right angle triangles.

120 cm: (30,40,50), (20,48,52), (24,45,51)

Given that L is the length of the wire, for how many values of L ≤ 1,500,000 can exactly one integer sided right angle triangle be formed?
*/

use fractions::{Fraction,stern_brocot_traversal};
use std::collections::{HashMap};

#[test]
fn solves() {
    assert_eq!(solve(1_500_000),161667);
}

fn main() {
    println!("Solution: {}", solve(1_500_000));
}

// euclid's formula for generating pythagorean triples: a^2 + b^2 = c^2
//a=k(m^2-n^2)/2, b=kmn, c=k(m^2+n^2)/2
//for m, n, k positive integers with m, n coprime and both odd
//generate all triples with a+b+c <= L
  //use sb tree to generate coprime pairs of m,n, then iterate on k
//tabulate the length of each one
//find entries with length = 1
fn solve(l: usize) -> usize {
    //map each length a+b+c to the number of pythagorean triples that have it
    let mut length_counts: HashMap <usize, usize> = HashMap::new();
    stern_brocot_traversal(
        &mut |frac: Fraction| {
            if frac.n % 2 == 1 && frac.d % 2 == 1 {
                let mut k = 1;
                let len = frac.d.pow(2) + frac.n*frac.d;
                while len*k <= l {
                    let count = length_counts.entry(len*k).or_insert(0);
                    *count += 1;
                    k += 1;
                }
    
            }
        },
        //only follow branches that produce triples meeting the length constraint
        &|frac: Fraction| {
            frac.d.pow(2) + frac.n*frac.d <= l

        }
    );
    return length_counts.values().filter(|&&v| v==1).count();
}

