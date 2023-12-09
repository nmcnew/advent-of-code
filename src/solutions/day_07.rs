use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
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
    Two = 1
}

#[derive(Debug)]
enum HandKind {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

pub fn part1(input: &String) -> i32 {

    return 0;
}
pub fn part2(input: &String) -> i32 {

    return 0;
}

fn compare_hands(hand1: Hand, hand2: Hand) -> i32{
    let mut diff = 0;
    return diff;
}

fn get_card_from_char(c: char) -> Card {
    let map = HashMap::from([('A', Card::Ace),
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
    ('2', Card::Two)
    ]);

    return map.get(&c).expect("Invalid Character provided").clone();

}
fn get_hand_value(cards: &Vec<Card>) -> HandKind {
    let map = HashMap::from([(vec![1,1,1,1,1], HandKind::HighCard),
    (vec![2,1,1,1], HandKind::OnePair),
    (vec![2,2,1], HandKind::TwoPair),
    (vec![3,1,1], HandKind::ThreeOfAKind),
    (vec![3,2], HandKind::FullHouse),
    (vec![4,1], HandKind::FourOfAKind),
    (vec![5], HandKind::FiveOfAKind),
    ]);


    return HandKind::HighCard;
}
fn compare_card(c1: char, c2: char) -> i32 {
    return 0;
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandKind
}

impl Hand {

    fn from_str(input: String) -> Hand {

        let mut cards = Vec::new();
        for c in input.chars(){
            cards.push(get_card_from_char(c));
        }
        let hand_type = get_hand_value(&cards);
        Hand {
            cards: cards,
            hand_type
        }
    }
}
#[test]
fn test_compare_hands() {

}
#[test]
fn part1_simple () {
    let input = String::from("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");

    let result = part1(&input);

    assert_eq!(6440, result);
}
