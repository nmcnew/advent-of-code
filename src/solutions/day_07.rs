use std::collections::HashMap;

enum Hands {
    Five = 8,
    Four = 7,
    FullHouse = 6,
    Three = 5,
    Two = 4,
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

fn compare_hands(hand1: String, hand2: String) -> i32{
    let mut diff = 0;
    let hand1_value = get_hand_value(hand1);
    let hand2_value = get_hand_value(hand2);
    for i in 0..hand1.len() {
        diff = compare_card(hand1.chars().nth(i).unwrap(), hand2.chars().nth(i).unwrap());
        if diff != 0 {
            break;
        }
    }
    return diff;
}
fn get_hand_value(hand: &String) -> Hands {
    let mut cards = HashMap::new();
    let hand_cards = hand.chars();
    for card in hand_cards {
        let count = cards.entry(card).or_insert(0);
        *count += 1;
    }
    return Hands::HighCard;
}
fn compare_card(c1: char, c2: char) -> i32 {
    return 0;
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
