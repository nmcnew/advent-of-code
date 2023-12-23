pub fn part1(input: &String) -> i32 {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let dists = lines
        .next()
        .unwrap()
        .replace("Distance: ", "")
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut result = 0;
    for i in 0..times.len() {
        let time = times[i];
        let record = dists[i];
        let mut times_beaten = 0;
        for speed in 0..time {
            let distance_traveled = (time - speed) * speed;
            if distance_traveled > record {
                times_beaten += 1;
            }
        }
        if result != 0 {
            result *= times_beaten;
        } else {
            result = times_beaten;
        }
    }
    return result;
}

pub fn part2(input: &String) -> u64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let record = lines
        .next()
        .unwrap()
        .replace("Distance: ", "")
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let mut times_beaten = 0;
    for speed in 0..time {
        let distance_traveled = (time - speed) * speed;
        if distance_traveled > record {
            times_beaten += 1;
        }
    }
    return times_beaten;
}
