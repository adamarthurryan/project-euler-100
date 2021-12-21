use digits;

fn main() {
    let mut max_sum = 0;

    for a in 1..100 {
        let mut a_b = digits::to_digits(a);  
        for b in 1..100 {
            a_b = digits::digits_mul(&a_b, &digits::to_digits(a));
            let sum = digits::digit_sum(&a_b);

            if sum>max_sum {
                println!("New max: digit_sum({}^{}) = {}", a, b, sum);
                max_sum = sum;
            }
        }
    }
}

fn digit_sum(n:u64) -> u64 {
    digits::digit_sum(&digits::to_digits(n))
}
