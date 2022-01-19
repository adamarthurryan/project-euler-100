
/*
A spider, S, sits in one corner of a cuboid room, measuring 6 by 5 by 3, and a fly, F, sits in 
the opposite corner. By travelling on the surfaces of the room the shortest "straight line" 
distance from S to F is 10 and the path is shown on the diagram.

However, there are up to three "shortest" path candidates for any given cuboid and the 
shortest route doesn't always have integer length.

It can be shown that there are exactly 2060 distinct cuboids, ignoring rotations, with integer
dimensions, up to a maximum size of M by M by M, for which the shortest route has integer length 
when M = 100. This is the least value of M for which the number of solutions first exceeds two thousand; 
the number of solutions when M = 99 is 1975.

Find the least value of M such that the number of solutions first exceeds one million.
*/

use fractions::{Fraction,SternBrocotFractions};
use std::cmp::{max, min};

#[test]
fn cuboids_works() {
    assert_eq!(cuboids(5),3);
    assert_eq!(cuboids(100),2060);
    assert_eq!(cuboids(99),1975);
}

#[test]
fn solves() {
    assert_eq!(solve(2_000), 100);
    assert_eq!(solve(1_000_000), 1818);
}

fn main() {
    //binary search for the solution
    println!("Solution: {}", solve(1_000_000));
}

//binary search for the solution
//(since there isn't an easy way to calculate cuboid(M) incrementally)
fn solve(M:usize) -> usize {
    let mut i=1;
    let mut step= 1000;
    loop {
        let val = cuboids(i);
        if val>=M && step==1 {
            return i;
        }

        if val<M {
            i+=step;
        }
        if val>M {
            i-=step;
            step /= 2;
        }
    }
}

fn sb_mediant_a(mediant: Fraction) -> usize {
    (mediant.d.pow(2)-mediant.n.pow(2))/2
}
fn sb_mediant_b(mediant: Fraction) -> usize {
    mediant.n*mediant.d
}
fn sb_mediant_c(mediant: Fraction) -> usize {
    (mediant.d.pow(2)+mediant.n.pow(2))/2
}

/*
euclid's formula for generating pythagorean triples: a^2 + b^2 = c^2
a=k(m^2-n^2)/2, b=kmn, c=k(m^2+n^2)/2
for m, n, k positive integers with m, n coprime and both odd
generate all triples with a,b,c < M  and a > b
use sb tree to generate coprime pairs of m,n, then iterate on k
*/
fn cuboids(M: usize) -> usize {
    //set up the generator of coprime m,n
    let branch_test = move |mediant: Fraction| sb_mediant_a(mediant) <= 2*M && sb_mediant_b(mediant) <= 2*M;
    let fractions = SternBrocotFractions::new(branch_test);

    //only consider odd m,n
    let fractions = fractions.filter(|frac| frac.n % 2 == 1 && frac.d % 2 == 1);


    //for each emitted fraction, count the number of rectangular solids that can be generated from it
    let mut count = 0;
    for frac in fractions {
        
        //generate the primitive pythagorean triple from m,n
        let (a,b,c) = (sb_mediant_a(frac), sb_mediant_b(frac), sb_mediant_c(frac));

        //iterate on multiples of the triple
        let mut k = 1;
        while (k*a <= M && k*b <= 2*k*a) || (k*a <= 2*k*b && k*b <= M)   {
            //add all possible rectangular solids
        
            // some careful counting...
            if k*a <= M && k*b <= 2*k*a {
                count += count_em(k*a,k*b,M);
            }
            if k*b <= M && k*a <= 2*k*b {
                count += count_em(k*b,k*a,M);
            }

            k += 1;
        }
    }
    count
}

fn count_em(a: usize, b:usize, M:usize) -> usize {
    if !(a<=M && b <=2*a) {
        return 0;
    }

    let low = max(1, max((b) as isize - (a) as isize, (b) as isize - M as isize) ) as usize;
    let high = min(M+1,min(a + 1, b));
    if (low<high) {
        (high-low+1)/2
    }
    else {
        0
    }
}
