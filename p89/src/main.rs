
/*
The 11K text file, roman.txt (right click and 'Save Link/Target As...'), contains one thousand 
numbers written in valid, but not necessarily minimal, Roman numerals; see About... 
Roman Numerals for the definitive rules for this problem.

Find the number of characters saved by writing each of these in their minimal form.
*/

#[test]
fn solves() {
    assert_eq!(solve(), 743);
}
fn main() {
    println!("Solution: {} ", solve());
}

fn solve() -> usize {
    let lines = include_str!("p089_roman.txt").split('\n');

    let mut saved_chars = 0;
    for line in lines {
        let num = parse_roman(line);
        let roman = emit_roman(num);

        saved_chars += line.len() - roman.len();
    }
    
    saved_chars
}


fn parse_roman(s: &str) -> usize {
//not working
    //let str = String::from_str(str).unwrap();
    let mut num = 0;

    let mut chars = s.chars().peekable();
    while let Some(this) = chars.next() {
        let next = if let Some(&c) = chars.peek() {c} else {'Z'};
        match (this,next) {
            ('C', 'M') => { num += 900; chars.next(); }
            ('C', 'D') => { num += 400; chars.next(); }
            ('X', 'C') => { num += 90; chars.next(); }
            ('X', 'L') => { num += 40; chars.next(); }
            ('I', 'X') => { num += 9; chars.next(); }
            ('I', 'V') => { num += 4; chars.next(); }
            ('M', _) => { num += 1000; }           
            ('D', _) => { num += 500; }          
            ('C', _) => { num += 100; }            
            ('L', _) => { num += 50; }            
            ('X', _) => { num += 10; }            
            ('V', _) => { num += 5; }            
            ('I', _) => { num += 1; }      
            _ => { panic!("Invalid roman numeral string: {}", s) }
        };
    }

    num
}

const VALS: [usize;13] = [1000, 900, 500, 400,  100, 90,   50,  40,   10,   9,   5,   4,    1];
const STRS: [&str; 13] = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];

fn emit_roman(n: usize) -> String {
    let mut str = String::new();
    let mut n = n;

    for i in 0..VALS.len() {
        while n >= VALS[i] {
            n -= VALS[i];
            str.push_str(STRS[i]);
        }
    }

    str
}