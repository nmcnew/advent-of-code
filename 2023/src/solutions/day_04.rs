use std::str::FromStr;

pub fn part1(input: &String) -> i32 {
    let result = input
        .lines()
        .map(|line| Card::from_str(line).unwrap())
        .map(|c| 1 << c.win_count >> 1)
        .sum();
    return result;
}

pub fn part2(input: &String) -> i32 {
    let mut cards = input
        .lines()
        .map(|line| Card::from_str(line).unwrap())
        .collect::<Vec<_>>();

    for card_id in 0..cards.len() {
        let mut limit = card_id + cards[card_id].win_count as usize;
        if limit >= cards.len() {
            limit = cards.len() - 1;
        }
        for n in card_id + 1..=limit {
            cards[n].count = cards[n].count + cards[card_id].count;
        }
    }
    let result = cards.iter().map(|c| c.count).sum();
    return result;
}

fn vec_of_nums(input: &str) -> Vec<i32> {
    return input
        .split(" ")
        .filter(|n| !n.is_empty())
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
}

#[derive(Debug, PartialEq)]
struct Card {
    game_id: i32,
    win_count: i32,
    winning_numbers: Vec<i32>,
    scratched_numbers: Vec<i32>,
    count: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Card {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut thing = s.split("|");
        let mut game_and_winners = thing
            .next()
            .ok_or(ParsePointError)?
            .trim()
            .split(":")
            .filter(|s| !s.is_empty());
        let game_id = game_and_winners.next().ok_or(ParsePointError)?[4..]
            .trim()
            .parse::<i32>()
            .map_err(|_| ParsePointError)?;
        let winning_numbers = vec_of_nums(game_and_winners.next().ok_or(ParsePointError)?);
        let scratched_numbers = vec_of_nums(thing.next().ok_or(ParsePointError)?);
        let win_count = scratched_numbers
            .iter()
            .map(|c| if winning_numbers.contains(c) { 1 } else { 0 })
            .sum();
        return Ok(Card {
            game_id: game_id,
            win_count,
            winning_numbers,
            scratched_numbers,
            count: 1,
        });
    }
}
