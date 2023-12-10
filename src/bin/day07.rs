use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input/day07.txt");
    part1(input);
    part2(input);
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Card {
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
    Ace = 13,
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Card2 {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Queen = 11,
    King = 12,
    Ace = 13,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: i32,
}

#[derive(Debug)]
struct Hand2 {
    cards: Vec<Card2>,
    hand_type: HandType,
    bid: i32,
}

fn part1(input: &str) {
    let mut winning = 0;

    let mut hands: Vec<Hand> = input.lines().map(|line| line_to_hand(line)).collect();

    hands.sort_by(|h1, h2| {
        let ordering = h1.hand_type.cmp(&h2.hand_type);
        if ordering != Ordering::Equal {
            return ordering;
        } else {
            for i in 0..5 {
                if h1.cards[i] != h2.cards[i] {
                    return h1.cards[i].cmp(&h2.cards[i]);
                }
            }

            return Ordering::Equal;
        }
    });

    for (i, h) in hands.iter().enumerate() {
        winning += (i + 1) as i32 * h.bid;
    }

    println!("Part 1: {winning}");
}

fn part2(input: &str) {
    let mut winning = 0;

    let mut hands: Vec<Hand2> = input.lines().map(|line| line_to_hand2(line)).collect();

    hands.sort_by(|h1, h2| {
        let ordering = h1.hand_type.cmp(&h2.hand_type);
        if ordering != Ordering::Equal {
            return ordering;
        } else {
            for i in 0..5 {
                if h1.cards[i] != h2.cards[i] {
                    return h1.cards[i].cmp(&h2.cards[i]);
                }
            }

            return Ordering::Equal;
        }
    });

    for (i, h) in hands.iter().enumerate() {
        winning += (i + 1) as i32 * h.bid;
    }

    println!("Part 2: {winning}");
}

fn line_to_hand(line: &str) -> Hand {
    let cards: Vec<Card> = line[0..5].chars().map(|c| char_to_card(c)).collect();
    Hand {
        hand_type: hand_type(&cards),
        cards,
        bid: (&line)[6..].parse().unwrap(),
    }
}

fn line_to_hand2(line: &str) -> Hand2 {
    let cards: Vec<Card2> = line[0..5].chars().map(|c| char_to_card2(c)).collect();
    Hand2 {
        hand_type: hand_type2(&cards),
        cards,
        bid: (&line)[6..].parse().unwrap(),
    }
}

fn char_to_card(c: char) -> Card {
    match c {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => Card::Jack,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => panic!("Invalid card")
    }
}

fn char_to_card2(c: char) -> Card2 {
    match c {
        '2' => Card2::Two,
        '3' => Card2::Three,
        '4' => Card2::Four,
        '5' => Card2::Five,
        '6' => Card2::Six,
        '7' => Card2::Seven,
        '8' => Card2::Eight,
        '9' => Card2::Nine,
        'T' => Card2::Ten,
        'J' => Card2::Joker,
        'Q' => Card2::Queen,
        'K' => Card2::King,
        'A' => Card2::Ace,
        _ => panic!("Invalid card")
    }
}

fn hand_type(cards: &Vec<Card>) -> HandType {
    let mut card_counts = HashMap::new();
    for card in cards {
        card_counts.insert(
            card,
            match card_counts.get(&card) {
                Some(count) => count + 1,
                None => 1
            },
        );
    }

    let &max_same_type = card_counts.values().max().unwrap();

    match card_counts.len() {
        1 => HandType::FiveOfAKind,
        2 => if max_same_type == 4 {
            HandType::FourOfAKind
        } else {
            HandType::FullHouse
        },
        3 => if max_same_type == 3 {
            HandType::ThreeOfAKind
        } else {
            HandType::TwoPair
        },
        4 => HandType::OnePair,
        _ => HandType::HighCard
    }
}

fn hand_type2(cards: &Vec<Card2>) -> HandType {
    let mut card_counts = HashMap::new();
    for card in cards {
        card_counts.insert(
            card,
            match card_counts.get(&card) {
                Some(count) => count + 1,
                None => 1
            },
        );
    }

    let joker_count: i32 = card_counts.iter()
        .filter(|(&c, _count)| *c == Card2::Joker)
        .map(|c| c.1)
        .sum();

    let max_same_type = if joker_count < 5 {
        *(card_counts.iter()
            .filter(|(&c, _count)| *c != Card2::Joker)
            .map(|c| c.1)
            .max().unwrap())
    } else {
        0
    };

    let non_joker_type_count = card_counts.iter()
        .filter(|(&c, _count)| *c != Card2::Joker).count();

    match max_same_type + joker_count {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => if non_joker_type_count < 3 {
            HandType::FullHouse
        } else {
            HandType::ThreeOfAKind
        }
        2 => if non_joker_type_count < 4 {
            HandType::TwoPair
        } else {
            HandType::OnePair
        }
        _ => HandType::HighCard
    }
}
