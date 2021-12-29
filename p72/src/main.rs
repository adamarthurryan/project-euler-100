use primes::TotientSieve;

#[test]
fn solves() {
    assert_eq!(solve(8),21);
    assert_eq!(solve(1_000_000),303963552391);
}

fn main() {
    println!("solve(8): {}", solve(8));
    println!("solve(12,000): {}", solve(12_000));
    println!("Solution: {}", solve(1_000_000));
}

fn solve(max_d: usize) -> usize {
    let sieve = TotientSieve::new(max_d);
    return (2..=max_d).map(|d| sieve.totient(d)).sum();
}
