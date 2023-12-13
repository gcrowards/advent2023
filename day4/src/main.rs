use std::fs;

fn main() {
    println!("Hello, world!");

    // a();

    b();
}

/// A
fn a() {
    let mut sum = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let all_numbers = line.split(":").collect::<Vec<&str>>()[1]
            .split("|")
            .collect::<Vec<&str>>();
        let winners: Vec<u32> = all_numbers[0]
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let mut winner_count = 0;
        for line_number in all_numbers[1].split_ascii_whitespace() {
            if winners.contains(&line_number.parse::<u32>().unwrap()) {
                winner_count = winner_count + 1;
            }
        }

        if winner_count > 0 {
            sum = sum + 2u32.pow(winner_count - 1);
        }
    }
    println!("Sum: {}", sum);
}

/// B
fn b() {
    let file = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let mut card_counts = vec![1u32; lines.len()];
    for (i, &line) in lines.iter().enumerate() {
        // Find how many winners there are.
        let all_numbers = line.split(":").collect::<Vec<&str>>()[1]
            .split("|")
            .collect::<Vec<&str>>();
        let winners: Vec<u32> = all_numbers[0]
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let mut winner_count = 0;
        for line_number in all_numbers[1].split_ascii_whitespace() {
            if winners.contains(&line_number.parse::<u32>().unwrap()) {
                winner_count = winner_count + 1;
            }
        }

        // println!("Winner count: {}. {:?}", winner_count, card_counts);

        // If there are winners, add counts to those cards.
        if winner_count > 0 {
            // How many of this card did we have?
            let card_count = card_counts[i];
            for j in i + 1..i + 1 + winner_count {
                card_counts[j as usize] = card_counts[j as usize] + card_count;
            }
        }
    }

    println!("Sum: {}", card_counts.iter().sum::<u32>());
}
