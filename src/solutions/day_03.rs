use std::collections::HashMap;

pub fn part1(input: &String) -> i32 {
    let schematic = build_schematic(input);
    let mut char_map: HashMap<(i32, i32), char> = HashMap::new();
    let mut numbers: Vec<(i32, i32, i32, i32)> = Vec::new();
    for i in 0..schematic.len() {
        let mut number = String::new();
        let mut num_start = 0;
        let mut in_number = false;
        for j in 0..schematic[i].len() {
            let position = schematic[i][j];
            if position.is_digit(10) {
                if !in_number {
                    in_number = true;
                    num_start = j;
                }
                number.push(position);
            } else if position != '.' {
                in_number = false;
                char_map.insert((i.try_into().unwrap(), j.try_into().unwrap()), position);
            } else {
                in_number = false;
            }
            if (!in_number || j == schematic[i].len() - 1) && number.len() > 0 {
                let num = number.parse::<i32>().unwrap();
                let num_end = j - 1;
                numbers.push((
                    num,
                    i.try_into().unwrap(),
                    num_start.try_into().unwrap(),
                    num_end.try_into().unwrap(),
                ));
                number = String::new();
            }
        }
    }

    let result: i32 = numbers
        .iter()
        .filter(|num| {
            for x in num.1 - 1..=num.1 + 1 {
                for y in num.2 - 1..=num.3 + 1 {
                    let entry = char_map.get(&(x, y));
                    if entry.is_some() {
                        return true;
                    }
                }
            }
            return false;
        })
        .map(|scheme| scheme.0)
        .sum();
    return result;
}

pub fn part2(input: &String) -> i32 {
    let schematic = build_schematic(input);
    let mut gear_map: Vec<(i32, i32)> = Vec::new();
    let mut numbers: Vec<(i32, i32, i32, i32)> = Vec::new();
    for i in 0..schematic.len() {
        let mut number = String::new();
        let mut num_start = 0;
        let mut in_number = false;
        for j in 0..schematic[i].len() {
            let position = schematic[i][j];
            if position.is_digit(10) {
                if !in_number {
                    in_number = true;
                    num_start = j;
                }
                number.push(position);
            } else if position == '*' {
                in_number = false;
                gear_map.push((i.try_into().unwrap(), j.try_into().unwrap()));
            } else {
                in_number = false;
            }
            if (!in_number || j == schematic[i].len() - 1) && number.len() > 0 {
                let num = number.parse::<i32>().unwrap();
                let num_end = j - 1;
                numbers.push((
                    num,
                    i.try_into().unwrap(),
                    num_start.try_into().unwrap(),
                    num_end.try_into().unwrap(),
                ));
                number = String::new();
            }
        }
    }
    let numbers_gears = numbers.iter()
    .map(|num| (num, gear_map.iter().filter(|(gear_x, gear_y)| *gear_x >= num.1-1 && *gear_x <= num.1+1 && *gear_y >= num.2-1 && *gear_y <= num.3+1).collect::<Vec<_>>()))
    .filter(|ng| ng.1.len() == 1)
    .map(|ng| (**ng.1.iter().next().unwrap(),*ng.0)).collect::<Vec<_>>();

    let mut result = 0;
    let numbers_gears2 = numbers_gears.clone();
    for ng in numbers_gears {
        let adj = numbers_gears2.iter().filter(|ng2| ng2.0 == ng.0 && ng2.1 != ng.1).next();
        if adj.is_some() {
            let a = adj.expect("lol I already checked");
            result += ng.1.0 * a.1.0;
        }
    }
    return result/2;
}

fn build_schematic(input: &String) -> Vec<Vec<char>> {
    let schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    return schematic;
}
