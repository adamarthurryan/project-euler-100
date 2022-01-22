use itertools::Itertools;
use regex::Regex;

fn main() {
    println!("Solution: {}", solve());
}



fn solve() -> usize {
    let words = Regex::new(r" and | the | for | but ").unwrap();
    let encoded = parse(include_str!("p059_cipher.txt"));

    let keys = (0..3).map(|_| 'a'..='z').multi_cartesian_product().map(|a| a.into_iter().collect::<String>());

    for key in keys {
        let decoded = decode(&encoded, &key);
        let conditioned = condition(&decoded);
        
        let matches = words.find_iter(&conditioned);
        if matches.count()>3 {
            println!("{}", key);
            println!("{}", decoded);

            return decoded.chars().map(|c| c as usize).sum();
        }
    }

    panic!("No solution found");
}

fn index_of_confidence(str: &str) -> f64 {
    let mut histogram = [0;26];
    for c in str.chars() {
        histogram[c as usize - 97] += 1;
    }

    let len = str.len() as f64;
    let mut ic = 0.0;
    for h in histogram {
        let h = h as f64;
        ic += h/len*(h-1.0)/(len-1.0);
    }

    ic * 26.0
}

fn condition(str: &str) -> String {
    str.to_lowercase().chars()
        .map(|c| if c=='.' || c==',' || c=='?' || c=='!' || c==':' || c==';' { ' ' } else { c })
        .filter(|&c| ('a'<=c && c<='z') || c==' ')
        .collect::<String>()
}

fn decode(data: &[u8], password: &str) -> String {
    let chars = password.chars().map(|c| c as u8).cycle();
    let zip = data.iter().zip(chars);

    zip.map(|(&val, char)| (val^char) as char).collect()
}

fn parse(s: &str) -> Vec<u8> {
    s.split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect()
}