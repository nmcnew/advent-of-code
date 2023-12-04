pub fn part1(input: &String) -> i32 {
    let result = input
        .lines()
        .map(|line| {
            get_winning_spots(line)
                .iter()
                .fold(1, |acc, x| if *x { acc << 1 } else { acc })
                >> 1
        })
        .sum();
    return result;
}

pub fn part2(input: &String) -> i32 {
    let mut cards = input
        .lines()
        .map(|line| {
            get_winning_spots(line)
                .iter()
                .fold(0, |acc, x| if *x { acc + 1 } else { acc })
        })
        .map(|v| (v, 1))
        .collect::<Vec<_>>();

    for card_id in 0..cards.len() {
        let mut limit = card_id + cards[card_id].0;
        if limit >= cards.len() {
            limit = cards.len() - 1;
        }
        for n in card_id + 1..=limit {
            cards[n].1 = cards[n].1 + cards[card_id].1;
        }
    }
    let result = cards.iter().map(|c| c.1).sum();
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

fn get_winning_spots(input: &str) -> Vec<bool> {
    let mut card = input.split("|");
    let own_numbers = card.next().unwrap();
    let winners = card.next().unwrap();
    let card_values = vec_of_nums(own_numbers.split(":").last().unwrap());
    let winner_values = vec_of_nums(winners.trim());
    return card_values
        .iter()
        .map(|c| winner_values.contains(c))
        .collect::<Vec<_>>();
}
