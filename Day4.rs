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
    // Star 1 - just count the number of winning numbers with scoring.
    // if score > 0 {
    //     return 2_usize.pow(score - 1);
    // }
    // assert!(score == 0);
    // return 0_usize;

    // Star 2 - Scratchcards that win scratch cards.
    return score as usize;
}

fn main() -> std::io::Result<()> {
    let mut cards: Vec<String> = Vec::new();
    {
        // Is this Rust-ic?
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            panic!("Usage: {} <input file>", args[0]);
        }
        let fname: &str = &args[1];
        let mut file: File = File::open(fname)?;
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)?;
        for line in contents.lines() {
            cards.push(line.to_string());
        }
    }

    let mut duplicated_scratch_cards: Vec<(String, usize)> = cards
        .iter()
        .map(|s| (s.clone(), 1))
        .collect::<Vec<(String, usize)>>();

    let mut total_score: usize = 0;

    for index in 0..duplicated_scratch_cards.len() {
        let (card, num_of_cards) = duplicated_scratch_cards[index].clone();
        let preamble_index = card.find(':').unwrap();
        let split_index = card.find('|').unwrap();
        let winning_numbers = card[preamble_index + 1..split_index]
            .split(|c: char| !c.is_digit(10))
            .filter(|s: &&str| !s.is_empty())
            .filter_map(|s: &str| s.parse::<usize>().ok())
            .collect::<HashSet<usize>>();
        let got_numbers = card[split_index + 1..]
            .split(|c: char| !c.is_digit(10))
            .filter(|s: &&str| !s.is_empty())
            .filter_map(|s: &str| s.parse::<usize>().ok())
            .collect::<Vec<usize>>();
        // Star 1 - just count the number of winning numbers.
        // total_score += scratch_card_points(&winning_numbers, &got_numbers);

        // Star 2 - Scratchcards that win scratch cards.
        let num_of_winning_cards: usize = scratch_card_points(&winning_numbers, &got_numbers);
        total_score += num_of_cards;
        for added_index in 1..=num_of_winning_cards {
            duplicated_scratch_cards[index+added_index].1 += num_of_cards;
        }
    }
    println!("Total Score: {}", total_score);

    Ok(())
}