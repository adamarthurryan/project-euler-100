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
        assert_eq!(is_palindrome(&to_digits(35262)), false);
        assert_eq!(is_palindrome(&to_digits(35253)), true);
    }

    #[test]
    fn digit_sums() {
        assert_eq!(digit_sum(&to_digits(35253)), 18);
    }
    #[test]
    fn add() {
        assert_eq!(from_digits(&digits_add(&to_digits(35253), &to_digits(453223))), 488476);
        assert_eq!(from_digits(&digits_add(&to_digits(84894), &to_digits(453223))), 538117);
        assert_eq!(from_digits(&digits_add(&to_digits(87), &to_digits(7))), 94);
        assert_eq!(from_digits(&digits_add(&to_digits(9), &to_digits(99))), 108);
    }
    #[test]
    fn mul() {
        assert_eq!(from_digits(&digits_mul(&to_digits(0), &to_digits(999))), 0);
        assert_eq!(from_digits(&digits_mul(&to_digits(1), &to_digits(1))), 1);
        assert_eq!(from_digits(&digits_mul(&to_digits(9), &to_digits(9))), 81);
        assert_eq!(from_digits(&digits_mul(&to_digits(9), &to_digits(900))), 8100);
        assert_eq!(from_digits(&digits_mul(&to_digits(123), &to_digits(456))), 56088);
        assert_eq!(from_digits(&digits_mul(&to_digits(123456789), &to_digits(123456789))), 15241578750190521);
    }
}

pub fn digit_sum(n: &Vec<u64>) -> u64 {
    n.iter().sum()
}

pub fn is_palindrome(n: &Vec<u64>) -> bool {
    n.iter().zip(n.iter().rev()).take(n.len()/2+1).all(|(a,b)| a==b)
}

//return the base 10 digits in a number, least to most significant
pub fn to_digits(n: u64) -> Vec<u64> {
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
pub fn from_digits(digits:&Vec<u64>) -> u64 {
    let mut n=0;
    for d in digits.iter().rev() {
        n = n*10 + d;
    }

    return n;
}

//simple digit-wise addition
//could be generalized to a list of numbers
pub fn digits_add(a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
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
pub fn digits_mul(a: &[u64], b: &[u64]) -> Vec<u64> {
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