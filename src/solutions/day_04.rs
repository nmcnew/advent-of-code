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
        let card = &cards[card_id];
        let card_count = card.count;
        let mut limit = card_id + card.win_count as usize;
        if limit >= cards.len() {
            limit = cards.len() - 1;
        }
        for n in card_id + 1..=limit {
            cards[n].count = cards[n].count + card_count;
        }
    }
    let result = cards.iter().map(|c| c.count).sum();
    return result;
}

fn vec_of_nums(input: &str) -> Vec<i32> {
    return input
        .trim()
        .split(" ")
        .filter(|n| !n.is_empty())
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
}

fn get_times_won(winning_numbers: Vec<i32>, own_numbers: Vec<i32>) -> i32 {
    return own_numbers
        .iter()
        .map(|c| if winning_numbers.contains(c) { 1 } else { 0 })
        .sum();
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
        let winning_string = game_and_winners.next().ok_or(ParsePointError)?;
        let own_numbers = thing.next().ok_or(ParsePointError)?;
        let winning_numbers = vec_of_nums(winning_string);
        let scratched_numbers = vec_of_nums(own_numbers);
        let win_count = get_times_won(winning_numbers.clone(), scratched_numbers.clone());
        return Ok(Card {
            game_id: game_id,
            win_count,
            winning_numbers,
            scratched_numbers,
            count: 1,
        });
    }
}
