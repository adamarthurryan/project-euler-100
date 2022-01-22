/*
The cube, 41063625 (345^3), can be permuted to produce two other cubes: 56623104 (384^3) and 66430125 (405^3). In fact, 41063625 is the smallest cube which has exactly three permutations of its digits which are also cube.

Find the smallest cube for which exactly five permutations of its digits are cube.
*/

use itertools::Itertools;
use digits::Digits;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_count_3() {
        assert_eq!(solve(3), Some(41063625));
    }
    #[test]
    fn solves_count_5() {
        assert_eq!(solve(5), Some(127035954683));
    }
}


fn main() {
    let solution = solve(5);

    println!("Solution: {}", solution.expect("No solution found!"));
}

fn solve(count: usize) -> Option<usize> {
    
    //the list of cubic numbers
    let cubes = (1..).map(|n: usize| n.pow(3)).map(Digits::new);
    //grouped by number of digits
    let cubes_by_len = cubes.group_by(|n_digits| n_digits.len());
    
    //the list of cubic numbers, grouped by number of digits
    //let cubes = (1..).map(|n: usize| n.pow(3)).group_by(|&n| (n as f64).log10() as usize);



    for cubes_l in &cubes_by_len {
        println!("Checking cubes of length {}", cubes_l.0);
        
        //maps each permutation (recorded in sorted digit order) to its occurence count and first occurence 
        let mut perms: HashMap<Digits, (usize, Digits)> = HashMap::new();

        for n_digits in cubes_l.1 {
            let mut n_sorted = n_digits.clone();
            n_sorted.sort();
            
            let entry = perms.entry(n_sorted).or_insert((0, n_digits.clone()));
            entry.0 += 1;

            if entry.0 == count {
                let found = entry.1.to_num();
                println!("Found cube with {} permutations: {}", count, found);
                return Some(found);
            }
        } 
    }


    None

}
