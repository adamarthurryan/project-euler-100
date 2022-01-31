

/* These are the pairings of numbers that must occur.
Each digit must be found on at least one die face, and adjacent digits must be found on the opposite die.

For this problem, one die has a 2 and the other a 5,
so we can just count combinations of 5 sided die and multiply by 2 to get the full six sides.

    0 — 4
    | \ |
8 — 1 — 6 — 3

    2 — 5

This must be some graph-theoretic problem but I'm not sure what. Something about forests of spanning subtrees?
Trees that span every edge in the graph? Spanning trees of the line graph / conjugate of the graph?
Something about edge covers?

How many complete bipartite graphs K6,6 can be formed st each edge in the graph is included at least once? 

Algorithm:

- consider all orientations of the edges in the above graph
- for each orientation, gather the set of in-vertex labels and out-vertex labels and place them on die a and die b respectively
- then count the 

*/

use std::{fmt, ops::RangeBounds};

fn main() {
    println!("Solution: {}", solve());    
}

#[derive(Clone, Debug)]
struct DicePair {
    a: Vec<u8>,
    b: Vec<u8>,
    not_a: Vec<u8>,
    not_b: Vec<u8>
}

impl DicePair {
    pub fn new() -> Self {
        DicePair {a: vec![], b: vec![], not_a: vec![], not_b: vec![]}
    }
}

impl fmt::Display for DicePair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let write_digits = |f: &mut fmt::Formatter<'_>, digits: &[u8]| {
            for d in digits {
                write!(f, "{}", d);
            }
        };

        write!(f, "(");
        write_digits(f, &self.not_a);
        write!(f, "/");
        write_digits(f, &self.a);
        write!(f, ", ");
        write_digits(f, &self.not_b);
        write!(f, "/");
        write_digits(f, &self.b);
        write!(f, ")")
    }
}

const DIGITS: [u8;6] = [0,1,3,4,6,8];
fn solve() -> usize {
    let mut dice_pairs = vec![DicePair::new()];
    for digit in DIGITS {
        dice_pairs = possible(&dice_pairs, digit);
    }

    dice_pairs = dice_pairs.into_iter().filter(is_valid).collect();
    /*
    for pair in dice_pairs {
        println!("{}", pair);
    }
    */

    let possibilities = dice_pairs.iter().map(num_possible);

    possibilities.into_iter().sum()
}

fn num_possible(pair: &DicePair) -> usize {
    let mut count = 1;
    for (not, digits) in [(&pair.not_a, &pair.a), (&pair.not_b, &pair.b)] {
        //count number of additional digits that can be placed
        //note that not 6 also implies not 9
        count *= match 5-digits.len() {
            0 => 1,
            1 => 3,
            2 => 6,
            _ => 6
        };
        //count the number of ways 9 can be substituted for 6
        count *= if digits.contains(&6) {2} else {1};

        // this isn't accounting for the possibility that the a die could have both 6 and 9...
        if digits.contains(&6) {
            count += match 5-digits.len() {
                0 => 0,
                1 => 1,
                2 => 3,
                3 => 6,
                _ => 6
            }
        }
        
    }
    //count placement of 2-5 pair
    count *= 2;
    println!("{}: {}", count, pair);

    count
}

const PAIRS: [(u8,u8);7] = [(0,1), (0,4), (0,6), (1,6), (1,8), (3,6), (4,6)];

//returns true if the given pair of dice represents a valid configuration
fn is_valid(pair: &DicePair) -> bool {
    if pair.a.len()>5 || pair.b.len()>5 {
        return false;
    }
    
    for (c,d) in PAIRS {
        if !(pair.a.contains(&c) && pair.b.contains(&d)) && !(pair.b.contains(&c) && pair.a.contains(&d)) {
            return false;
        }
    }

    true
}

//generate all possible pairs of dice that can be formed from adding a single instance
//of the digit to one or the other or both dice
fn possible(possible_dice: &[DicePair], digit: u8) -> Vec<DicePair> {
    let mut new_dice = Vec::new();
    for dice in possible_dice {
        let (mut left,mut both,mut right) = (dice.clone(), dice.clone(), dice.clone());
        left.a.push(digit);
        left.not_b.push(digit);
        both.a.push(digit);
        both.b.push(digit);
        right.not_a.push(digit);
        right.b.push(digit); 

        //eliminate left/right symmetry
        if left.a!=right.b || left.b!=right.a {
            new_dice.push(right);
        }

        new_dice.push(left);
        new_dice.push(both);
    }
    new_dice
}

