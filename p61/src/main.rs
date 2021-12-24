fn main() {
    //generate all permutations of cycles of 6 4-digit numbers (lots of these)
    //or generate all 4-digit triangular ... octagonal etc. numbers and go from there 

    println!("Hello, world!");

    /*
    Triangle	 	P3,n=n(n+1)/2	 	1, 3, 6, 10, 15, ...
    Square	 	    P4,n=n2	 	1, 4, 9, 16, 25, ...
    Pentagonal	 	P5,n=n(3n−1)/2	 	1, 5, 12, 22, 35, ...
    Hexagonal	 	P6,n=n(2n−1)	 	1, 6, 15, 28, 45, ...
    Heptagonal	 	P7,n=n(5n−3)/2	 	1, 7, 18, 34, 55, ...
    Octagonal	 	P8,n=n(3n−2)	 	1, 8, 21, 40, 65, ...
    */

    let triangle =      (0..).map(|n| n*(n+1)/2).skip_while(|&x| x<1000).take_while(|&x| x<10000).collect::<Vec<i64>>();
    let squares =       (0..).map(|n| n*n).skip_while(|&x| x<1000).take_while(|&x| x<10000).collect::<Vec<i64>>();
    let pentagonal =    (0..).map(|n| n*(3*n-1)/2).skip_while(|&x| x<1000).take_while(|&x| x<10000).collect::<Vec<i64>>();
    let hexagonal =     (0..).map(|n| n*(2*n-1)).skip_while(|&x| x<1000).take_while(|&x| x<10000).collect::<Vec<i64>>();
    let heptagonal =    (0..).map(|n| n*(5*n-3)/2).skip_while(|&x| x<1000).take_while(|&x| x<10000).collect::<Vec<i64>>();
    let octagonal =     (0..).map(|n| n*(3*n-2)).skip_while(|&x| x<1000).take_while(|&x| x<10000).collect::<Vec<i64>>();

    let nums = [triangle, squares, pentagonal, hexagonal, heptagonal, octagonal];
    

    //build a graph?
    //look for a cycle of length n that visits each color?
    //pick a starting triangle

    //do something with these nums...

    println!("{:?}", nums);

    println!("--- {:?}", nums.iter().map(|l| l.len()).collect::<Vec<_>>());
    println!("possibilities {:?}", nums.iter().map(|l| l.len()).product::<usize>());
}

//concatenate the digits of b after the digits of a
fn concatenate(a:u64, b:u64) -> u64 {
    let exp = (b as f64).log10().ceil() as u32;
    return a*10u64.pow(exp)+b;
}


