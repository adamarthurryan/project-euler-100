use primal; 

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn solves_problem() {
        assert_eq!(solve(), 26241);
    }
}

fn main() {
    println!("Problem 58: Spiral Primes");
    println!("Solution: {}", solve());
}

fn solve() -> u64 {
    let sieve = primal::Sieve::new(1_000_000_000);

    let mut num_prime = 0f64;
    let mut num_total = 1f64;

    for n in 1..1_000_000_000_000 {
        let a = diags(n);
        for ai in a {
            if sieve.is_prime(ai as usize) {
                num_prime += 1f64;
            }
            num_total += 1f64;
        }
        if n > 1 && (num_prime / num_total) < 0.1 {
            println!("ratio of primes on diagonal below 10% at n:{}, side length:{}", n, n*2+1);
            return n*2+1;
        }

        if (n > 1 && n % 1000 == 0) || n == 3 {
            println!("ratio of primes on diagonal is {:0.5}% at n:{}, side length:{}", num_prime / num_total * 100f64, n, n*2+1);
        }
    }

    panic!("No solution found.")
    
}

// the diagonals are indexed from the center being 0
fn diags(n:u64) -> [u64; 4] {
    //kind of wonky handling of this edge case
    if n == 0 { return [1,1,1,1]}

    //bottom right diagonal is odd squares
    let mut corners: [u64;4] = [0,0,0,0];
    let lr = (n*2+1).pow(2);
    for i in 0..=3 {
        corners[i] = lr - n*2*(3-i as u64);
    }

    return corners;
}


