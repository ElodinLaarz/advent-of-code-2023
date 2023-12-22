use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

// Surely we'll have suits at some point...
type Hand = (Vec<String>, u32);

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
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
        "2" => Rank::Two,
        "3" => Rank::Three,
        "4" => Rank::Four,
        "5" => Rank::Five,
        "6" => Rank::Six,
        "7" => Rank::Seven,
        "8" => Rank::Eight,
        "9" => Rank::Nine,
        "T" => Rank::Ten,
        "J" => Rank::Jack,
        "Q" => Rank::Queen,
        "K" => Rank::King,
        "A" => Rank::Ace,
        _ => panic!("Invalid rank"),
    }
}

fn signature_to_hand_type(sig: Vec<usize>) -> HandType {
    match sig {
        ref v if *v == vec![1, 1, 1, 1, 1] => HandType::HighCard,
        ref v if *v == vec![2, 1, 1, 1] => HandType::OnePair,
        ref v if *v == vec![2, 2, 1] => HandType::TwoPair,
        ref v if *v == vec![3, 1, 1] => HandType::ThreeOfAKind,
        ref v if *v == vec![3, 2] => HandType::FullHouse,
        ref v if *v == vec![4, 1] => HandType::FourOfAKind,
        ref v if *v == vec![5] => HandType::FiveOfAKind,
        default => panic!("Invalid signature: {:?}", default),
    }
}

fn score_poker_hand(h: &Hand) -> HandType {
    let mut rank_counts: HashMap<Rank, usize> = HashMap::new();
    for card in h.0.iter() {
        let rank = value_to_rank(card);
        let mut count = rank_counts.entry(rank).or_insert(0);
        *count += 1;
    }
    let mut sorted_rc: Vec<usize> = rank_counts.values().cloned().collect::<Vec<usize>>();
    sorted_rc.sort_by(|a, b| b.cmp(a));
    let ht: HandType = signature_to_hand_type(sorted_rc);

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
