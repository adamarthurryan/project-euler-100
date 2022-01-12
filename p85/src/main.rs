


#[test]
fn a_works() {
    assert_eq!(a(3,1), 6);
    assert_eq!(a(3,2), 18);
}

#[test]
fn solves() {
    assert_eq!(solve(2_000_000), 2772);
}

fn main() {
    println!("Solution: {}", solve(2_000_000));
}

fn solve(count: usize) -> usize {
    //these are lower and upper bonds on the solution space 
    let lower = (1..).take_while(|&n| a(n,n)<count).last().unwrap();
    let upper = (1..).take_while(|&n| a(n,1)<count).last().unwrap();

    let count:isize = count as isize;

    //search the solution space for the n, m that minimizes count-a(n,m)
    let mut nearest_val = 0;
    let mut nearest_nm = (lower, lower);

    //this could be much more efficient
    //eg. searching only the margin between values under and over count
    for n in 1..=lower {
        for m in 1..=upper {
            let val = a(n,m) as isize;
            if (count-val).abs() < (count-nearest_val).abs() {
                nearest_val = val;
                nearest_nm = (n,m)
            }
        }
    }

    let (n,m) = nearest_nm;
    n*m 
}

//a(n,m) is the number of integer-sized rectangles within a rectangle of size n*m
fn a(n:usize, m:usize) -> usize {
    triangular(n)*triangular(m)
}

//triangular(n) is the nth triangular number = a(n,1)
fn triangular(n:usize) -> usize{
    n*(n+1)/2
}