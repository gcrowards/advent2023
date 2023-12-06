use std::fs;

fn main() {
    println!("Hello, world!");

    // A
    let mut sum = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let digits = line.chars().clone().map(|c| c.to_digit(10)).filter(|c| c.is_some());
        let digits: Vec<u32> = digits.into_iter().flatten().collect();
        let digit = digits[0] * 10 + digits.last().unwrap();
        println!("{}: {}", line, digit);
        sum = sum + digit;
    }
    println!("Sum: {}", sum);

    // B
}
