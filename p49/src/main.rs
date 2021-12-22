use permute::permute;
use itertools::Itertools; 
use primes;
use digits;

//this algo is not very efficient
//it works fine for four digit numbers, but takes several seconds on five digit numbers
fn main() {
    let seive = primes::Seive::new(9999);

    let mut options = Vec::new();

    //first gather all the permutations of each number
    //and filter for primes
    for i in 1000..=9999 {
        let a = permute(digits::to_digits(i));
        let mut a_primes: Vec<u64> = a.into_iter()
            .map(|ai| digits::from_digits(&ai))
            .filter(|&ai| ai >= 1000)
            .filter(|&ai| seive.is_prime(ai))
            .unique()
            .collect();

        a_primes.sort();

        options.push(a_primes);
    }

    //condition the resulting candidates - must be at least three permutations and remove non-unique options
    options = options.into_iter()
        .filter(|a| a.len()>=3)
        .unique()
        .collect();
        
    //now find all the permutations that are arithmetic series!
    for a in options {
        //get all three-item candidate series
        let candidates = choose_3_of(a);

        //test each candidate for being an arithmetic series
        for a in candidates {
            if a[1]-a[0] == a[2]-a[1] {
                //this is it!
                println!("Solution: {}, {}, {}", a[0], a[1], a[2]);
                println!("Answer: {}{}{}", a[0], a[1], a[2]);
            }
        }
    }

    //println!("{:?}", options);
}

fn choose_3_of(a:Vec<u64>) -> Vec<Vec<u64>> {
    let mut result: Vec<Vec<u64>> = Vec::new();
    for i in 0..a.len()-2 {
        for j in i+1..a.len()-1 {
            for k in j+1..a.len() {
                result.push(vec![a[i],a[j],a[k]]);
            }
        }
    }
    return result;
}
