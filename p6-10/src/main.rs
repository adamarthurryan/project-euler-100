use primal::{Primes};
use std::cmp::max;

#[test]
fn solves() {
    assert_eq!(solve_6(),25164150);
    assert_eq!(solve_7(),104743);
    assert_eq!(solve_8(),23514624000);
    assert_eq!(solve_9(),31875000);
    assert_eq!(solve_10(),142913828922);
}

fn main() {
    println!("Solution #6: {}", solve_6());
    println!("Solution #7: {}", solve_7());
    println!("Solution #8: {}", solve_8());
    println!("Solution #9: {}", solve_9());
    println!("Solution #10: {}", solve_10());
}

/*Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.*/
fn solve_6() -> usize {
    let sum_square = (1usize..=100).map(|n| n.pow(2)).sum::<usize>();
    let square_sum = (1..=100).sum::<usize>().pow(2);
    return square_sum-sum_square;
}

fn solve_7() -> usize {
    Primes::all().nth(10001 - 1).unwrap()
}

fn solve_8() -> usize {
    let num: Vec<usize> = P8_STRING.chars().filter(|&c| c>='0' && c<='9').map(|c| c.to_digit(10).unwrap() as usize).collect();

    //initialize with the first 13 elements
    let mut digits: Vec<usize> = num[0..13].to_vec();
    let mut max_prod = digits.iter().product();

    
    //try and find a bigger sum
    for i in 13..num.len() {
        digits = digits[1..].to_vec();
        digits.push(num[i]);
        max_prod = max(digits.iter().product(), max_prod);
    } 
    return max_prod;
}
fn solve_9() -> usize {
    //generate all 3-way partitions of 1000
    //and test whether they are triples
    for a in 1..=998 {
        for b in 1..=(1000-a-1) {
            let c = 1000 - a - b;
            let (a2,b2,c2) = ((a as usize).pow(2), (b as usize).pow(2), (c as usize).pow(2));
            if (a2+b2==c2 || a2+c2==b2 || b2+c2==a2) {
                return a*b*c;
            }
        }
    }
    panic!("No solution found");
}
fn solve_10() -> usize {
    Primes::all().take_while(|&p| p<2_000_000).sum()
}


const P8_STRING: &str = "73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450";