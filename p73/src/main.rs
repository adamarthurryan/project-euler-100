use fractions::{Fraction, SternBrocotFractions};



#[test]
fn solves() {
    assert_eq!(solve(Fraction::new(1,3), Fraction::new(1,2), 8),3);
    assert_eq!(solve(Fraction::new(1,3), Fraction::new(1,2), 12_000),7295372);
}

fn main() {
    println!("solve(8): {}", solve(Fraction::new(1,3), Fraction::new(1,2), 8));
    println!("Solution: {}", solve(Fraction::new(1,3), Fraction::new(1,2), 12_000));
}

fn solve(a: Fraction, b:Fraction,  max_d: usize) -> usize {
    let sb_fractions = SternBrocotFractions::with_range(|mediant| mediant.d<=max_d, a, b);
    return sb_fractions.count();
    //    return sb_tree_count(a,b,max_d);

}

//depth-first search of the Stern-Brocot tree
//counts all fractions between left and right with denominator at most max_d
//based on a discussion forum post by klopyrev
//this is fast but will overflow the stack quickly

fn sb_tree_count(left: Fraction, right: Fraction, max_d: usize) -> usize {
    let med = mediant(left, right);

    if med.d > max_d {
        return 0;
    }
    let mut count = 1;
    count += sb_tree_count(left, med, max_d);
    count += sb_tree_count(med, right, max_d);

    return count;
}

//calculates the mediant for use in the Stern-Brocot tree
fn mediant(a: Fraction, b: Fraction) -> Fraction {
    Fraction::new(a.n+b.n, a.d+b.d)
}

