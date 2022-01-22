#[test]
fn solves() {
    assert_eq!(solve(), 709);
}

fn main() {
    println!("Solution: {}", solve());
}


//a0^b0 < a1^b1 iff b0*log(a0) < b1*log(a1)
fn solve() -> usize {
    let mut data = parse(include_str!("p099_base_exp.txt"))
        .into_iter()
        .map(|v| (v[0], v[1]))
        .map(|(a,b)| (a as f64).ln()*(b as f64))
        .enumerate()
        .collect::<Vec<_>>();
    
    data.sort_by(|(_,fa), (_,fb)| fa.partial_cmp(fb).unwrap());

    data.last().unwrap().0 + 1
}

fn parse(s: &str) -> Vec<Vec<usize>> {
    s.split('\n')
        .map(|line| line.split(',').map(|num| num.parse::<usize>().unwrap()).collect())
        .collect()
}