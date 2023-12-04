pub fn part1(input: &String) -> i32 {
    let result: i32 = input
        .lines()
        .map(|line| {
            let winner_spots = get_winning_spots(line);
            return winner_spots
                .iter()
                .fold(1, |acc, x| if *x { acc << 1 } else { acc })
                >> 1;
        })
        .sum();
    return result;
}

pub fn part2(input: &String) -> i32 {
    let mut cards = input
        .lines()
        .map(|line| {
            let mut card = line.split("|");
            let card_values = vec_of_nums(card.next().unwrap().split(":").nth(1).unwrap());
            let winner_values = vec_of_nums(card.next().unwrap().trim());
            let next_cards = card_values.iter().fold(0, |acc, x| {
                if winner_values.contains(x) {
                    acc + 1
                } else {
                    acc
                }
            });
            return (next_cards, 1);
        })
        .collect::<Vec<_>>();

    for card_id in 0..cards.len() {
        let won_cards_count = cards[card_id].0;
        let number_of_me = cards[card_id].1;
        let next_cards_count = card_id + won_cards_count;
        let limit = if next_cards_count >= cards.len() {
            cards.len() - 1
        } else {
            next_cards_count
        };
        for n in card_id + 1..=limit {
            cards[n].1 = cards[n].1 + number_of_me;
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
    let card_values = vec_of_nums(own_numbers.split(":").nth(1).unwrap());
    let winner_values = vec_of_nums(winners.trim());
    return card_values
        .iter()
        .map(|c| winner_values.contains(c))
        .collect::<Vec<_>>();
}
