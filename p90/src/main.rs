

/* These are the pairings of numbers that must occur.
Each digit must be found on at least one die face, and adjacent digits must be found on the opposite die.


    0 — 4
    | \ |
8 — 1 — 6 — 3

    2 — 5


Note that every answer is a bipartite graph with each partite set a subset of {0..9} and the edges
covering the edges of the above graph. 

Also, or this problem, one die has a 2 and the other a 5,
so we can just count combinations of 5 sided die and multiply by 2 to get the full six sides.



*/

use digits::DigitalCounter;

#[test]
fn solves() {
    assert_eq!(solve(), 1217);
}

fn main() {
    println!("Solution: {}", solve());    
}

const PAIRS: [(usize,usize);8] = [(0,1), (0,4), (0,6), (1,6), (1,8), (3,6), (4,6), (2,5)];
const CRITICAL_DIGITS: [usize; 8] = [0,1,4,6,3,8,2,5];

type Die = Vec<usize>; 
type DicePair = (Die,Die);

///this algorithm is a bit involved, given that brute force is possible for this problem
///but it is relatively efficient
fn solve() -> usize {
    //for each of the critical digits, we either include it in the left die, the right die, or both
    //this iterator enumerates the choices
    let digit_choices = DigitalCounter::new(CRITICAL_DIGITS.len(), 3);

    //generate a dice pair for each choice and filter to valid solutions
    let dice_pairs = digit_choices.map(generate_dice_pair).filter(is_valid_pair);

    //the pairs will have some choice points -- switch 6 and 9, or add additional digits -- count these
    let possible_dice_pairs = dice_pairs.map(count_possible_dice);

    //each configuration is counted twice (l,r) and (r,l), so divide result by two
    possible_dice_pairs.sum::<usize>() / 2
}

///given a vector encoding the choices for inclusion of each critical digit in one or both dice
/// return the pair of dice that cooresponds 
fn generate_dice_pair(digit_choices: Vec<usize>) -> DicePair {
    //for each digit: 0 means it is in the left die only
    //                1 means it is in the right die only
    //                2 means it is in both dice
    let (mut left, mut right): DicePair = (Vec::new(), Vec::new());
    for (digit, choice) in CRITICAL_DIGITS.into_iter().zip(digit_choices) {
        match choice {
            0 => { left.push(digit); }
            1 => { right.push(digit); }
            _ => { left.push(digit); right.push(digit); }
        }
    }

    (left, right)
}

///count the possible ways the given basic pair can form a completely filled in pair of dice
/// - add non-critical digits
/// - consider switching the 6 for a 9 or having both a 6 and 9
fn count_possible_dice(pair: DicePair) -> usize {
    let mut count = 1;

    for die in [&pair.0, &pair.1] {
        if die.contains(&6) {
            
            if die.len() == 4 {
                //leave 6 and add 7,9  = 1 choice
                count *= 1;
            }
            else if die.len() == 5 {
                //  leave 6 add 1 of 7, 9     = 2 choices
                //  or switch 6/9 add 7       = 1 choice
                count *= 3;
            }
            else if die.len() == 6 {
                // switch 6/9       = 2 choices
                count *= 2;
            }
            else {
                panic!("Invalid die configuration {:?}", &pair);
            }
        }
        else {
            //if 6 is excluded, consider 9 to be already excluded
            //7 is added or not depending on the length
            count *= 1;
        }
    }

    count
}



///returns true if the given pair of dice represents a valid configuration
fn is_valid_pair(pair: &DicePair) -> bool {
    //println!("pair: {:?}", pair);
    let (a,b) = pair;

    //the dice must not have too many digits
    if a.len()>6 || b.len()>6 {
        return false;
    }
    
    //every edge in PAIRS must be represented at least once
    for (c,d) in PAIRS {
        if !(a.contains(&c) && b.contains(&d) || b.contains(&c) && a.contains(&d)) {
            return false;
        }
    }

    true
}