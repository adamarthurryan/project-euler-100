use std::convert::From;
use std::fmt;

use std::cmp::{Ord, PartialOrd, Ordering};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcf_works() {
        assert_eq!(gcf(9,6), 3);
        assert_eq!(gcf(19,6), 1);
        assert_eq!(gcf(60,24), 12);
    }

    #[test]
    fn sb_fractions_works() {
        let fracs: Vec<_> = SternBrocotFractions::new(4).collect();
        println!("fracs: {:?}",fracs);

        assert_eq!(fracs[0], Fraction::new(1,4));
        assert_eq!(fracs[1], Fraction::new(1,3));
        assert_eq!(fracs[2], Fraction::new(1,2));
        assert_eq!(fracs[3], Fraction::new(2,3));
        assert_eq!(fracs[4], Fraction::new(3,4));
    }


    #[test]
    fn into_f64_works() {
        let f:f64 = Fraction::new(3,8).into();
        assert_eq!(f, 3.0/8.0);
        let f:f64 = Fraction::new(7,81).into();
        assert_eq!(f, 7.0f64/81.0);
    }
    #[test]
    fn from_f64_works() {
        assert_eq!(Fraction::from_f64(0.5, 4),Fraction::new(2,4));
        assert_eq!(Fraction::from_f64(0.5, 3),Fraction::new(1,3));
        assert_eq!(Fraction::from_f64(0.5, 99),Fraction::new(49,99));
    }
    #[test]
    fn reduce_works() {
        assert_eq!(Fraction::new(2,4).is_reduced(),false);
        assert_eq!(Fraction::new(1,4).is_reduced(),true);
        assert_eq!(Fraction::new(2,4).reduce(),Fraction::new(1,2));
    }
    
}

pub fn gcf(a:usize, b:usize) -> usize {
    let (mut a, mut b) = if a<b { (b,a) } else { (a,b) };
    while b!=0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Fraction {
    pub n: usize,
    pub d: usize,
    pub sign: i8
}

impl Fraction {
    pub fn new(n: usize, d: usize) -> Self{
        //should check for invalid ie. d=0 fractions
        Fraction{n,d,sign:1}
    }

    //return the fraction the nearest equal to or below the given float
    //with the given denumerator
    pub fn from_f64(f: f64, d: usize) -> Self {
        let n = ((d as f64)*f).floor() as usize;
        Fraction::new(n,d)
    }

    pub fn is_reduced(&self) -> bool {
        gcf(self.n,self.d) == 1
    }

    pub fn reduce(&self) -> Self {
        let f = gcf(self.n, self.d);
        Fraction::new(self.n/f, self.d/f)
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.n*other.d).cmp(&(self.d*other.n))
    }
}
impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.n*other.d).cmp(&(self.d*other.n)))
    }
}

impl From<Fraction> for f64 {
    fn from(q: Fraction) -> f64 {
        q.sign as f64 * (q.n as f64 / q.d as f64)
    }
}

impl fmt::Debug for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.n, self.d))
    }
}


enum SternBrocotStackItem {
    Branch(Fraction,Fraction),
    Node(Fraction)
}

pub struct SternBrocotFractions {
    stack: Vec<SternBrocotStackItem>,
    max_denom: usize
}

impl SternBrocotFractions {
    //an iterator for all reduced fractions in the range 0/1 .. 1/1
    //with given maximum denominator 
    pub fn new(max_denom: usize) -> Self {
        SternBrocotFractions{max_denom, stack: vec![SternBrocotStackItem::Branch(Fraction::new(0,1), Fraction::new(1,1))]}
    }

    //an iterator for reduced fractions in the given range (exclusive)
    pub fn with_range(max_denom: usize, lower: Fraction, upper: Fraction ) -> Self {
        SternBrocotFractions{max_denom, stack: vec![SternBrocotStackItem::Branch(lower, upper)]}
    }
}

//iterator for the reduced fractions
impl Iterator for SternBrocotFractions {
    type Item = Fraction;


    // has to go all the way down the left branch
    // then start emitting and work it's way back around
     fn next(&mut self) -> Option<Self::Item> {
        while let Some(stack_item) = self.stack.pop() {
            match stack_item {
                //for Node items, return the mediant
                SternBrocotStackItem::Node(mediant) => { return Some(mediant) }
                //for Branch items, traverse the branch
                SternBrocotStackItem::Branch(left, right) => {
                    //calculate the mediant
                    let mediant = Fraction::new(left.n+right.n, left.d+right.d);
                    //if the mediant is in range, push the branches and node onto the stack
                    if mediant.d <= self.max_denom {
                        self.stack.push(SternBrocotStackItem::Branch(mediant, right));
                        self.stack.push(SternBrocotStackItem::Node(mediant));
                        self.stack.push(SternBrocotStackItem::Branch(left, mediant));
                    }
                }
            }
        }
        
        return None;
    }
}

//traverse the Stern-Brocot tree of reduced fractions with the given emit and branch functions
//emit will be called with every fraction that is found by the iterator in order
//branch will be called when each node is visited and should return true if the traversal should continue
//down that node or backtrack to the parent 
pub fn stern_brocot_traversal(emit: &mut dyn FnMut(Fraction) -> (), branch: &dyn Fn(Fraction) -> bool) {
    let mut stack: Vec<SternBrocotStackItem> = Vec::new();
    stack.push(SternBrocotStackItem::Branch(Fraction::new(0,1), Fraction::new(1,1)));   
    while let Some(stack_item) = stack.pop() {
        match stack_item {
            //for Node items, return the mediant
            SternBrocotStackItem::Node(mediant) => { emit(mediant) }
            //for Branch items, traverse the branch
            SternBrocotStackItem::Branch(left, right) => {
                //calculate the mediant
                let mediant = Fraction::new(left.n+right.n, left.d+right.d);
                //if the mediant is in range, push the branches and node onto the stack
                if branch(mediant) {
                    stack.push(SternBrocotStackItem::Branch(mediant, right));
                    stack.push(SternBrocotStackItem::Node(mediant));
                    stack.push(SternBrocotStackItem::Branch(left, mediant));
                }
            }
        }
    }
}
