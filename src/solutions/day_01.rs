use std::collections::HashMap;
pub fn run(input: &String) -> i32 {
    // Part 1
    let part1: i32 = input.lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<String>())
        .map(|number_string| [number_string.chars().nth(0).unwrap(),number_string.chars().nth(number_string.chars().count()-1).unwrap()].iter().collect::<String>())
        .map(|configNumber| configNumber.parse::<i32>().unwrap())
        .sum();
    let number_map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    let part2 = input.lines()
        .map(|line| {
            let char_enum = line.chars()
                .enumerate();
            let mut result = String::from("");
            for (i, c) in char_enum {
                let hash_value = number_map.iter().filter(|num| {
                    let end = i + num.0.len();
                    if end > line.len() {
                        return false;
                    } else if &line[i..end] == *num.0 {
                        return true;
                    }
                    return false;
                }).nth(0);
                if c.is_digit(10) {
                    result.push(c);
                } else if hash_value.is_some() {
                    result.push(*hash_value.unwrap().1);
                }
            }
            return result;
        })
        .map(|number_string| [number_string.chars().nth(0).unwrap(),number_string.chars().nth(number_string.chars().count()-1).unwrap()].iter().collect::<String>())
        .map(|configNumber| configNumber.parse::<i33>().unwrap())
        .sum();
    return part2;
}
