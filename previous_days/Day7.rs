use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

// Surely we'll have suits at some point...
type Hand = (Vec<String>, u32);

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Rank {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

// The HandTypes are ordered by strength, so we can just compare them.
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn value_to_rank(v: &str) -> Rank {
    match v {
        "J" => Rank::Joker,
        "2" => Rank::Two,
        "3" => Rank::Three,
        "4" => Rank::Four,
        "5" => Rank::Five,
        "6" => Rank::Six,
        "7" => Rank::Seven,
        "8" => Rank::Eight,
        "9" => Rank::Nine,
        "T" => Rank::Ten,
        "Q" => Rank::Queen,
        "K" => Rank::King,
        "A" => Rank::Ace,
        _ => panic!("Invalid rank"),
    }
}

// Given a hand type, what is the next best hand given a Joker?
fn upgrade_hand_type(ht: HandType) -> HandType {
    match ht {
        HandType::HighCard => HandType::OnePair,
        HandType::OnePair => HandType::ThreeOfAKind,
        HandType::TwoPair => HandType::FullHouse,
        HandType::ThreeOfAKind => HandType::FourOfAKind,
        HandType::FullHouse => HandType::FullHouse,
        HandType::FourOfAKind => HandType::FiveOfAKind,
        HandType::FiveOfAKind => HandType::FiveOfAKind,
    }
}

fn signature_to_hand_type(sig: Vec<usize>) -> HandType {
    match sig {
        ref v if sig.len() <= 5 && *v == vec![1, 1, 1, 1, 1][..sig.len()] => HandType::HighCard,
        ref v if sig.len() <= 4 && *v == vec![2, 1, 1, 1][..sig.len()] => HandType::OnePair,
        ref v if sig.len() <= 3 && *v == vec![2, 2, 1][..sig.len()] => HandType::TwoPair,
        ref v if sig.len() <= 3 && *v == vec![3, 1, 1][..sig.len()] => HandType::ThreeOfAKind,
        ref v if sig.len() <= 2 && *v == vec![3, 2][..sig.len()] => HandType::FullHouse,
        ref v if sig.len() <= 2 && *v == vec![4, 1][..sig.len()] => HandType::FourOfAKind,
        ref v if sig.len() <= 1 && *v == vec![5] => HandType::FiveOfAKind,
        _ => HandType::HighCard, // This could happen if hand all Jokers.
    }
}

fn score_poker_hand(h: &Hand) -> HandType {
    let mut rank_counts: HashMap<Rank, usize> = HashMap::new();
    let mut joker_count: usize = 0;
    for card in h.0.iter() {
        let rank = value_to_rank(card);
        if rank == Rank::Joker {
            joker_count += 1;
            continue;
        }
        let count = rank_counts.entry(rank).or_insert(0);
        *count += 1;
    }
    // Because Jokers Exist -- We need to find the best hand "upgrade".
    let mut sorted_rc: Vec<usize> = rank_counts.values().cloned().collect::<Vec<usize>>();
    sorted_rc.sort_by(|a, b| b.cmp(a));
    let mut ht: HandType = signature_to_hand_type(sorted_rc);
    for _ in 0..joker_count {
        ht = upgrade_hand_type(ht);
    }
    return ht;
}

fn compare_hands(h1: &Hand, h2: &Hand) -> std::cmp::Ordering {
    let h1_score = score_poker_hand(h1);
    let h2_score = score_poker_hand(h2);
    if h1_score < h2_score {
        return std::cmp::Ordering::Less;
    }
    if h1_score > h2_score {
        return std::cmp::Ordering::Greater;
    } 
    for (c1, c2) in h1.0.iter().zip(h2.0.iter()) {
        let c1_rank = value_to_rank(&c1);
        let c2_rank = value_to_rank(&c2);
        if c1_rank < c2_rank {
            return std::cmp::Ordering::Less;
        }
        if c1_rank > c2_rank {
            return std::cmp::Ordering::Greater;
        }
    }
    return std::cmp::Ordering::Equal;
}

fn parse_hand(s: &str) -> Hand {
    let mut hand: Hand = (Vec::new(), 0);
    // Hands look like 32T3K 765. The first five characters are the cards in
    // the hand, and space delimited value is the bet amount.
    let mut cards_and_bet = s.split(" ");
    let cards = cards_and_bet.next().unwrap();
    let bet = cards_and_bet.next().unwrap().parse::<u32>().unwrap();
    for card in cards.chars() {
        let value = card.to_string();
        hand.0.push(value);
    }
    hand.1 = bet;
    return hand;
}

fn sort_hands(hands: &Vec<Hand>) -> Vec<Hand> {
    let mut sorted_hands: Vec<Hand> = hands.clone();
    sorted_hands.sort_by(|a, b| compare_hands(a, b)); 
    return sorted_hands;
}

fn main() -> Result<(), Error> {
    // Read in the input file.
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filename: &str = args
        .get(1)
        .ok_or(Error::new(ErrorKind::Other, "No filename given"))?;
    let mut file: File = File::open(filename)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut hands: Vec<Hand> = Vec::new();
    // For each line, parse the hand and bet.
    for line in lines {
        if line == "" {
            continue;
        }
        let hand = parse_hand(line);
        hands.push(hand);
    }
    // Totally order the hands (ranking the poorest hand as 1).
    let sorted_hands = sort_hands(&hands);
    for hand in &sorted_hands {
        println!("hand: {:?}, hand_type: {:?}", hand, score_poker_hand(&hand));
    }
    let mut total: usize = 0;
    for index in 0..sorted_hands.len() {
        let hand = &sorted_hands[index];
        total += (index + 1) * hand.1 as usize;
    }
    println!("Total: {}", total);
    Ok(())
}
