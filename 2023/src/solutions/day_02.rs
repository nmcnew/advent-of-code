pub fn part1(input: &String) -> i32 {
    let red = 12;
    let green = 13;
    let blue = 14;
    let result: i32 = input
        .lines()
        .map(|line| {
            let mut game_result = line.split(":");
            let game_id = game_result
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let games = game_result.next().unwrap().trim().split(";");
            let pulls = games
                .map(|game| {
                    game.trim()
                        .split(",")
                        .map(|block| {
                            (
                                block
                                    .trim()
                                    .split(" ")
                                    .next()
                                    .unwrap()
                                    .parse::<i32>()
                                    .unwrap(),
                                block.trim().split(" ").last().unwrap(),
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            return (game_id, pulls);
        })
        .collect::<Vec<_>>()
        .iter()
        .filter(|game| {
            game.1.iter().all(|rounds| {
                !rounds.iter().any(|round| match round.1 {
                    "red" => round.0 > red,
                    "blue" => round.0 > blue,
                    "green" => round.0 > green,
                    _ => panic!("WHAT THE"),
                })
            })
        })
        .map(|game| game.0)
        .sum();
    return result;
}

pub fn part2(input: &String) -> i32 {
    let result: i32 = input
        .lines()
        .map(|line| {
            let mut game_result = line.split(":");
            let game_id = game_result
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let games = game_result.next().unwrap().trim().split(";");
            let pulls = games
                .map(|game| {
                    game.trim()
                        .split(",")
                        .map(|block| {
                            (
                                block
                                    .trim()
                                    .split(" ")
                                    .next()
                                    .unwrap()
                                    .parse::<i32>()
                                    .unwrap(),
                                block.trim().split(" ").last().unwrap(),
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            return (game_id, pulls);
        })
        .map(|game| {
            game.1
                .iter()
                .map(|rounds| {
                    rounds.iter().fold((0, 0, 0), |acc, x| match x.1 {
                        "red" => (x.0, acc.1, acc.2),
                        "green" => (acc.0, x.0, acc.2),
                        "blue" => (acc.0, acc.1, x.0),
                        _ => panic!("what did you do..."),
                    })
                })
                .fold((0, 0, 0), |mut acc, x| {
                    if acc.0 < x.0 {
                        acc.0 = x.0;
                    }
                    if acc.1 < x.1 {
                        acc.1 = x.1;
                    }
                    if acc.2 < x.2 {
                        acc.2 = x.2;
                    }
                    acc
                })
        })
        .map(|min_blocks| min_blocks.0 * min_blocks.1 * min_blocks.2)
        .sum();

    return result;
}
