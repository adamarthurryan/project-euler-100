use std::ops::Add;
use std::ops::Mul;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_digits() {
        let result = from_digits(&to_digits(35262));
        assert_eq!(result, 35262);
    }
    
    #[test]
    fn palindromes() {
        assert_eq!(Digits::new(35262).is_palindrome(), false);
        assert_eq!(Digits::new(35253).is_palindrome(), true);
    }

    #[test]
    fn digit_sums() {
        assert_eq!(Digits::new(35253).digit_sum(), 18);
    }
    #[test]
    fn digit_length_works() {
        assert_eq!(digit_length(33), 2);
        assert_eq!(digit_length(100), 3);
        assert_eq!(digit_length(99), 2);
    }
    #[test]
    fn add() {

        assert_eq!((Digits::new(35253) + Digits::new(453223)).to_num(), 488476);
        assert_eq!((Digits::new(84894) + Digits::new(453223)).to_num(), 538117);
        assert_eq!((Digits::new(87) + Digits::new(7)).to_num(), 94);
        assert_eq!((Digits::new(9) + Digits::new(99)).to_num(), 108);
    }
    #[test]
    fn mul() {
        assert_eq!((Digits::new(0) * Digits::new(999)).to_num(), 0);
        assert_eq!((Digits::new(1) * Digits::new(1)).to_num(), 1);
        assert_eq!((Digits::new(9) * Digits::new(9)).to_num(), 81);
        assert_eq!((Digits::new(9) * Digits::new(900)).to_num(), 8100);
        assert_eq!((Digits::new(123) * Digits::new(456)).to_num(), 56088);
        assert_eq!((Digits::new(123456789) * Digits::new(123456789)).to_num(), 15241578750190521);
    }
    #[test]
    fn pow() {
        assert_eq!(Digits::new(3).pow(2).to_num(), 9);
        assert_eq!(Digits::new(4).pow(4).to_num(), 256);
        assert_eq!(Digits::new(1).pow(1).to_num(), 1);
        assert_eq!(Digits::new(3).pow(0).to_num(), 1);
        assert_eq!(Digits::new(5).pow(4).to_num(), 625);
    }

}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Digits {
    base: usize, //unused so far
    digits: Vec<usize>
}

impl Digits {
    pub fn new(n: usize) -> Self {
        Digits{base:10, digits:to_digits(n)}
    }

    pub fn from_digits(digits: Vec<usize>) -> Self {
        Digits{base:10, digits}
    }

    pub fn digit_sum(&self) -> usize {
        self.digits.iter().sum()
    }
    
    pub fn is_palindrome(&self) -> bool {
        self.digits.iter().zip(self.digits.iter().rev()).take(self.digits.len()/2+1).all(|(a,b)| a==b)
    }

    pub fn to_num(&self) -> usize {
        from_digits(&self.digits)
    }

    pub fn to_string(&self) -> String {
        self.digits.iter().rev().fold(String::new(), |a, b| a + &b.to_string())
    }

    pub fn digits(&self) -> &Vec<usize> {
        &self.digits
    }

    pub fn digits_mut(&mut self) -> &mut Vec<usize> {
        &mut self.digits
    }

    pub fn sort(&mut self) {
        self.digits.sort();
    }

    pub fn len(&self) -> usize {
        self.digits.len()
    }

    pub fn pow(&self, e: usize) -> Self{
        let mut result = Digits::new(1);
        for _i in 0..e {
            //not sure why we need to clone self
            //should be just able to deref... doesn't need to be moved
            result = result * self.clone();
        }

        return result;
    }
}

impl Add for Digits {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        if self.base != other.base {
            panic!("Attempt to add digits with different bases");
        }

        Self {
            base: self.base,
            digits: digits_add(&self.digits, &other.digits),
        }
    }
}
    
impl Mul for Digits {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        if self.base != other.base {
            panic!("Attempt to multiply digits with different bases");
        }
        
        Self {
            base: self.base,
            digits: digits_mul(&self.digits, &other.digits),
        }
    }
}

//return the base 10 digits in a number, least to most significant
fn to_digits(n: usize) -> Vec<usize> {
    if n==0 { return vec![]; }
    let mut a = Vec::new();
    let mut n = n;
    while n>0 {
        a.push(n%10);
        n = n/10;
    }

    return a;
}

//returns the number represented by the given digits in base 10
fn from_digits(digits:&[usize]) -> usize {
    let mut n=0;
    for d in digits.iter().rev() {
        n = n*10 + d;
    }

    return n;
}

//simple digit-wise addition
//could be generalized to a list of numbers
fn digits_add(a: &[usize], b: &[usize]) -> Vec<usize> {
    //let a be the longer of a, b
    let (a,b) = if b.len() > a.len() { (b, a) }
    else { (a, b) };

    //accumulator
    let mut sum = Vec::with_capacity(a.len()+1);
    let mut rem = 0;

    //sum digits from a and b
    for i in 0..b.len() {
        let mut c = a[i]+b[i]+rem;
        rem = 0;
        if c >= 10 {
            rem = 1;
            c = c - 10;
        }

        sum.push(c);
    }

    //sum digits in a only
    for i in b.len()..a.len() {
        let mut c = a[i]+rem;
        rem = 0;
        if c >= 10 {
            rem = 1;
            c = c - 10;
        }

        sum.push(c);
    }

    //include remainder
    if rem > 0 {
        sum.push(rem);
    }

    return sum;
}

//katasuba-ish multiplication
//xy = x1*y1*10^2m + (x1*y0 + x0*y1)*10^m + x0*y0
//where x = x1*10^m + x0 and y = y1*10^m + y0
pub fn digits_mul(a: &[usize], b: &[usize]) -> Vec<usize> {
    //let a be the longer of a, b
    let (a,b) = if b.len() > a.len() { (b, a) }
        else { (a, b) };
    
    //handle 0-length and 1-length cases
    if b.len() == 0 { return vec![]; }
    if b.len() == 1 {
        let b = b[0];
        let mut prod = Vec::with_capacity(a.len()+1);
        let mut rem = 0;
        for ai in a {
            let ci = ai*b + rem;
            //this divide op is not fast, but i guess it doesn't happen so much
            rem = ci/10; 
            prod.push(ci%10);
        }

        if rem > 0 {
            prod.push(rem)
        }

        return prod;
    }

    //pick m 
    let m = b.len() / 2;

    //split a and b
    let (a0,a1) = (&a[..m], &a[m..]);
    let (b0,b1) = (&b[..m], &b[m..]);
    
    //divide and conquor multiplication
    let mut z2 = digits_mul(a1,b1);
    let mut z1 = digits_add(&digits_mul(a1, b0), &digits_mul(a0, b1));
    let z0 = digits_mul(a0,b0);

    //prepend i*m 0s to zi
    z2.splice(0..0, vec![0; 2*m]);
    z1.splice(0..0, vec![0; m]);

    //combine zi into a single number
    return digits_add(&z2, &digits_add(&z1, &z0));
}

pub fn digit_length(n:usize) -> usize {
    ((n+1) as f64).log10().ceil() as usize
}