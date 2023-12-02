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
    .collect::<Vec<_>>()
    .iter()
    .map(|game| game.1.iter().fold(game.1.first(), |acc, x| ));

    return 0;
}
