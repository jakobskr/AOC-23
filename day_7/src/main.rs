use std::{cmp::Ordering, char};
use std::collections::HashMap;
use helpers;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind
}

fn main() {
    let path = helpers::abs_path() + "/inputs/in";
    println!("path: {}", path);
    let problem : Vec<String> = helpers::input_to_vec(&path, false);
    println!("part a: {}", part_a(&problem));
    println!("part b: {}", part_b(&problem));
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Hand {
    hand: String,
    bet: usize,
    hand_type: HandType,
    joker : bool
}

fn card_value (c : char, j : bool) -> usize {
    if c.is_numeric() {
        return c.to_digit(10).unwrap() as usize;
    }

    if c == 'A' {
        return 14;
    }
    if c == 'K' {
        return 13;
    }
    if c == 'Q' {
        return 12;
    }
    if c == 'J' {
        if j {
            return 0;
        }
        return 11;
    }
    if c == 'T' {
        return 10;
    }
    
    return 0;
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {

        if self.hand_type as usize == other.hand_type as usize {
            let mut i = 0;
            let a_cards : Vec<char> = self.hand.chars().collect();
            let b_cards : Vec<char> = other.hand.chars().collect();
            while i < self.hand.len() {
                if a_cards[i] == b_cards[i] {
                    i += 1;
                    continue;
                }

                return card_value(a_cards[i], self.joker).cmp( &card_value(b_cards[i], self.joker));
            }
            return Ordering::Equal
        }

        if self.hand_type as usize > other.hand_type as usize {
            return Ordering::Greater;
        }

        return Ordering::Less
    }
}

impl PartialOrd for Hand
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}

fn find_hand_type (hand: &str, joker : bool) -> HandType {
    let mut char_map : HashMap<char, usize> = HashMap::new();

    for c in hand.chars() {
        *char_map.entry(c).or_insert(0) += 1;
    }


    let len = char_map.len();
    let mut vals: Vec<&usize> =  char_map.values().collect();
    vals.sort();
    vals.reverse();

    if len == 1 {
        return HandType::FiveKind;
    }

    if len == 2 {

        if joker {
            if hand.contains('J') {
                return HandType::FiveKind;
            }
        }

        if *vals[0] == 4 {
            return HandType::FourKind;
        }

        if *vals[0] == 3 {
            return HandType::FullHouse;
        }
    }

    if len == 3 {
        if *vals[0] == 2 {

            if joker {
                if hand.contains('J') {
                    if hand.chars().filter(|x| *x == 'J').collect::<Vec<char>>().len() == 1 {
                        return HandType::FullHouse;
                    }
                    return HandType::FourKind;
                }
            }

            return HandType::TwoPair;
        }

        if *vals[0] == 3 {

            if joker {
                if hand.contains('J') {
                    return HandType::FourKind;
                }   
            }

            return HandType::ThreeKind;
        }
    }

    if len == 4{
        if joker {
            if hand.contains('J') {
                return HandType::ThreeKind;
            }
        }
        return HandType::OnePair;
    }

    if len == 5 {
        if joker {
            if hand.contains('J') {
                return HandType::OnePair;
            }
        }
        return HandType::HighCard;
    }

    return HandType::HighCard;
}

fn part_a(problem : &Vec<String>) -> usize {
    
    let mut hands : Vec<Hand> = vec![];
    
    for line in problem {
        let line_vec : Vec<&str> = line.split_whitespace().collect();
        
        hands.push(Hand {
            hand : line_vec[0].to_string(),
            bet : line_vec[1].parse().unwrap(),
            hand_type : find_hand_type(line_vec[0], false),
            joker: false
        });
    }

    hands.sort();

    let mut i = 0;
    let mut sum = 0;

    while i < hands.len() {
        sum += hands[i].bet * (i + 1);
        
        i+= 1;
        
    }

    return sum
}

fn part_b(problem : &Vec<String>) -> usize {
    let mut hands : Vec<Hand> = vec![];
    
    for line in problem {
        let line_vec : Vec<&str> = line.split_whitespace().collect();
        
        hands.push(Hand {
            hand : line_vec[0].to_string(),
            bet : line_vec[1].parse().unwrap(),
            hand_type : find_hand_type(line_vec[0], true),
            joker: true
        });
    }

    hands.sort();
    
    let mut i = 0;
    let mut sum = 0;

    while i < hands.len() {
        sum += hands[i].bet * (i + 1);
        
        i+= 1;
        
    }

    return sum
}