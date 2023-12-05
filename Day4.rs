use std::collections::hash_set::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn scratch_card_points(winning_numbers: &HashSet<usize>, got_numbers: &[usize]) -> usize {
    let mut score: u32 = 0;
    for number in got_numbers {
        if winning_numbers.contains(number) {
            score += 1;
        }
    }
    if score > 0 {
        return 2_usize.pow(score - 1);
    }
    assert!(score == 0);
    return 0_usize;
}


fn main() -> std::io::Result<()> {
    let mut cards: Vec<Vec<char>> = Vec::new();
    {
        // Is this Rust-ic?
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            panic!("Usage: {} <input file>", args[0]);
        }
        let fname: &String = &args[1];
        let mut file: File = File::open(fname)?;
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)?;
        for line in contents.lines() {
            cards.push(line.chars().collect::<Vec<char>>());
        }
    }
    
    let mut total_score: usize = 0;

    for card in cards {
        let preamble_index = card.iter().position(|&c| c == ':').unwrap();
        let split_index = card.iter().position(|&c| c == '|').unwrap();
        let winning_numbers = card[preamble_index+1..split_index]
            .iter()
            .collect::<String>()
            .split(|c: char| !c.is_digit(10))
            .filter(|s: &&str| !s.is_empty()) // What? Why &&str?
            .filter_map(|s: &str| s.parse::<usize>().ok())
            .collect::<HashSet<usize>>();
        let got_numbers = card[split_index+1..]
            .iter()
            .collect::<String>()
            .split(|c: char| !c.is_digit(10))
            .filter(|s: &&str| !s.is_empty())
            .filter_map(|s: &str| s.parse::<usize>().ok())
            .collect::<Vec<usize>>();
        total_score += scratch_card_points(&winning_numbers, &got_numbers);
    }
    println!("Total Score: {}", total_score);

    Ok(())
}