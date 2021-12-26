/*Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is called a reduced proper fraction.

If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:

1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8

It can be seen that 2/5 is the fraction immediately to the left of 3/7.

By listing the set of reduced proper fractions for d ≤ 1,000,000 in ascending order of size, find the numerator of the fraction immediately to the left of 3/7.
*/

use fractions::Fraction;

#[test]
fn solves() {
    assert_eq!(solve(8),2);
    assert_eq!(solve(1_000_000),428570);
}

fn main() {
    println!("solve(8): {}", solve(Fraction::new(3,7),8));
    println!("Solution: {}", solve(Fraction::new(3,7), 1_000_000));
}

fn solve(search: Fraction, max_d: usize) -> usize {
    
    //the search space is all fractions in the cantor enumeration of fractions
    //on the slope n/d

    //imagine a d*d unit grid
    //take the point (n,d)
    //draw a line from (n,d) to the origin
    //any cells that are intersected by this line are candidates for the cells adjacent to n/d
    //if the centerpoint of the cell is above the line, is is less than n/d
    //if it is below the line, it is more than n/d
    //the cell closest to the line is most adjacent...


    //this could be more elegant and accurate by not using a slope but instead just 
    let slope: f64 = search.into();
    let fracs = (2..=max_d).map(|di| Fraction::from_f64(slope, di)).filter(|q| q.is_reduced());
    let diffs = fracs.filter(|q| q!=&search);
    let q = diffs.max_by(|q0,q1| q0.cmp(q1)).unwrap();
    
    return q.n;   
}

