use std::collections::HashMap;

pub fn part1(_input: &str) -> i32 {
    let hand_bets = _input
        .lines()
        .map(|f| {
            let split = f.split(" ").collect::<Vec<_>>();
            (Hand::from_str(split[0]), split[1].parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();
    let result = get_result(hand_bets);
    return result;
}

pub fn part2(_input: &str) -> i32 {
    let hand_bets = _input
        .lines()
        .map(|f| {
            let split = f.split(" ").collect::<Vec<_>>();
            (
                Hand::from_str_wildcard(split[0]),
                split[1].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    return get_result(hand_bets);
}

fn get_result(mut hand_bets: Vec<(Hand, i32)>) -> i32 {
    hand_bets.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result = 0;
    for i in 0..hand_bets.len() {
        let hand_bet = &hand_bets[i];
        let rank: i32 = i as i32 + 1;
        result += hand_bet.1 * rank;
    }
    return result;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Ace = 13,
    King = 12,
    Queen = 11,
    Jack = 10,
    Ten = 9,
    Nine = 8,
    Eight = 7,
    Seven = 6,
    Six = 5,
    Five = 4,
    Four = 3,
    Three = 2,
    Two = 1,
    Joker = 0,
}

impl Card {
    fn from_char(c: char) -> Card {
        let map = HashMap::from([
            ('A', Card::Ace),
            ('K', Card::King),
            ('Q', Card::Queen),
            ('J', Card::Jack),
            ('T', Card::Ten),
            ('9', Card::Nine),
            ('8', Card::Eight),
            ('7', Card::Seven),
            ('6', Card::Six),
            ('5', Card::Five),
            ('4', Card::Four),
            ('3', Card::Three),
            ('2', Card::Two),
        ]);

        return map.get(&c).expect("Invalid Character provided").clone();
    }
    fn from_char_wildcard(c: char) -> Card {
        let map = HashMap::from([
            ('A', Card::Ace),
            ('K', Card::King),
            ('Q', Card::Queen),
            ('J', Card::Joker),
            ('T', Card::Ten),
            ('9', Card::Nine),
            ('8', Card::Eight),
            ('7', Card::Seven),
            ('6', Card::Six),
            ('5', Card::Five),
            ('4', Card::Four),
            ('3', Card::Three),
            ('2', Card::Two),
        ]);

        return map.get(&c).expect("Invalid Character provided").clone();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandKind {
    fn value_from_int_pattern(input: Vec<i32>) -> HandKind {
        let map = HashMap::from([
            (vec![1, 1, 1, 1, 1], HandKind::HighCard),
            (vec![2, 1, 1, 1], HandKind::OnePair),
            (vec![2, 2, 1], HandKind::TwoPair),
            (vec![3, 1, 1], HandKind::ThreeOfAKind),
            (vec![3, 2], HandKind::FullHouse),
            (vec![4, 1], HandKind::FourOfAKind),
            (vec![5], HandKind::FiveOfAKind),
        ]);

        return *map
            .get(&input)
            .expect("Could not find value from given input");
    }
    fn from_cards(cards: &Vec<Card>) -> HandKind {
        let mut card_map = HashMap::new();

        for card in cards {
            let card_val = card_map.entry(card).or_insert(0);
            *card_val += 1;
        }
        let mut sorted_result: Vec<i32> = card_map.values().map(|f| *f).collect();
        sorted_result.sort();
        sorted_result.reverse();
        let map_result = HandKind::value_from_int_pattern(sorted_result);
        return map_result;
    }

    fn from_cards_wildcard(cards: &Vec<Card>) -> HandKind {
        let mut card_map = HashMap::new();
        let mut wildcards = 0;
        for card in cards {
            match card {
                Card::Joker => {
                    wildcards += 1;
                }
                _ => {
                    let card_val = card_map.entry(card).or_insert(0);
                    *card_val += 1;
                }
            }
        }
        let mut sorted_result: Vec<i32> = card_map.values().map(|f| *f).collect();
        if sorted_result.len() == 0 {
            sorted_result.push(5);
        } else {
            sorted_result.sort();
            sorted_result.reverse();
            sorted_result[0] += wildcards;
        }
        let map_result = HandKind::value_from_int_pattern(sorted_result);
        return map_result;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandKind,
}

impl Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut diff: i32 = self.hand_type as i32 - other.hand_type as i32;
        if diff == 0 {
            for i in 0..self.cards.len() {
                diff = self.cards[i] as i32 - other.cards[i] as i32;
                if diff != 0 {
                    break;
                }
            }
        }

        if diff > 0 {
            return std::cmp::Ordering::Greater;
        } else if diff < 0 {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Equal;
        }
    }

    fn from_str_wildcard(input: &str) -> Hand {
        let mut cards = Vec::new();
        for c in input.chars() {
            cards.push(Card::from_char_wildcard(c));
        }
        let hand_type = HandKind::from_cards_wildcard(&cards);
        Hand {
            cards: cards,
            hand_type,
        }
    }

    fn from_str(input: &str) -> Hand {
        let mut cards = Vec::new();
        for c in input.chars() {
            cards.push(Card::from_char(c));
        }
        let hand_type = HandKind::from_cards(&cards);
        Hand {
            cards: cards,
            hand_type,
        }
    }
}

#[test]
fn hand_parse() {
    let result = Hand::from_str("32T3K");
    let expected = Hand {
        cards: vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
        hand_type: HandKind::OnePair,
    };
    assert_eq!(expected.cards, result.cards);
    assert_eq!(expected.hand_type, result.hand_type);
}

#[test]
fn hand_order() {
    let mut hands = vec![
        Hand::from_str("32T3K"),
        Hand::from_str("T55J5"),
        Hand::from_str("KK677"),
        Hand::from_str("KTJJT"),
        Hand::from_str("QQQJA"),
    ];
    hands.sort_by(|a, b| a.cmp(b));
    let expected = vec![
        Hand::from_str("32T3K"),
        Hand::from_str("KTJJT"),
        Hand::from_str("KK677"),
        Hand::from_str("T55J5"),
        Hand::from_str("QQQJA"),
    ];
    assert_eq!(expected, hands);
}
#[test]
fn part1_simple() {
    let input = String::from(
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
    );

    let result = part1(&input);

    assert_eq!(6440, result);
}

#[test]
fn part2_simple() {
    let input = String::from(
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
    );

    let result = part2(&input);

    assert_eq!(5905, result);
}
