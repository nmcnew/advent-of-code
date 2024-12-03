use std::collections::HashMap;

pub fn part1(input: &str) -> i32 {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for l in input.split("\n") {
        if l.is_empty() {
            continue;
        }
        let (left, right) = l.trim().split_once("   ").unwrap();
        list1.push(left.parse::<i32>().unwrap());
        list2.push(right.parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();
    let mut result = 0;
    for i in 0..list1.len() {
        result += (list1[i] - list2[i]).abs();
    }

    result
}

pub fn part2(input: &str) -> i32 {
    let mut list1 = Vec::new();
    let mut occurences = HashMap::<i32, i32>::new();
    for l in input.split("\n") {
        if l.is_empty() {
            continue;
        }
        let (left, right) = l.trim().split_once("   ").unwrap();

        list1.push(left.parse::<i32>().unwrap());
        let right_num = right.parse::<i32>().unwrap();
        if let Some(count) = occurences.get_mut(&right_num) {
            *count += 1;
        } else {
            occurences.insert(right_num, 1);
        }
    }
    let mut result = 0;
    for x in list1.iter() {
        if let Some(occurence) = occurences.get(x) {
            result += x * occurence;
        }
    }
    result
}

#[test]
fn part1_test() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    let result = part1(input);

    assert_eq!(result, 11);
}
#[test]
fn part2_test() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    let result = part2(input);

    assert_eq!(result, 31);
}
