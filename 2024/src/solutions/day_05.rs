use std::collections::HashMap;

use petgraph::Graph;

pub fn part1(input: &str) -> i32 {
    let (ordering, updates) = input.split_once("\n\n").unwrap();
    let mut map = HashMap::<i32, Vec<i32>>::new();
    ordering.split("\n").for_each(|x| {
        let (before_string, after_string) = x.split_once("|").unwrap();
        let before = before_string.parse::<i32>().unwrap();
        let after = after_string.parse::<i32>().unwrap();
        if let Some(result) = map.get_mut(&before) {
            if !result.contains(&after) {
                result.push(after);
            }
        } else {
            map.insert(before, vec![after]);
        }
    });
    let mut final_result = 0;
    for x in updates.split("\n") {
        if x.is_empty() {
            continue;
        }
        let page_updates = x
            .split(",")
            .map(|n| {
                dbg!(&n);
                n.trim().parse::<i32>().unwrap()
            })
            .collect::<Vec<_>>();
        let middle_value = page_updates[page_updates.len() / 2];
        let safe = check_updates(&page_updates, &map);
        if safe {
            final_result += middle_value;
        }
    }
    return final_result;
}

pub fn part2(input: &str) -> i32 {
    let (ordering, updates) = input.split_once("\n\n").unwrap();
    let mut map = HashMap::<i32, Vec<i32>>::new();
    ordering.split("\n").for_each(|x| {
        let (before_string, after_string) = x.split_once("|").unwrap();
        let before = before_string.parse::<i32>().unwrap();
        let after = after_string.parse::<i32>().unwrap();
        if let Some(result) = map.get_mut(&before) {
            if !result.contains(&after) {
                result.push(after);
            }
        } else {
            map.insert(before, vec![after]);
        }
    });
    let mut unsafe_updates = Vec::new();
    for x in updates.split("\n") {
        if x.is_empty() {
            continue;
        }
        let page_updates = x
            .split(",")
            .map(|n| {
                dbg!(&n);
                n.trim().parse::<i32>().unwrap()
            })
            .collect::<Vec<_>>();
        let safe = check_updates(&page_updates, &map);
        if !safe {
            unsafe_updates.push(page_updates);
        }
    }
    let mut final_result = 0;
    for x in unsafe_updates.iter() {
        let fixed_update = fix_update(x, &map);
        let middle_value = fixed_update[fixed_update.len() / 2];
        final_result += middle_value;
    }
    return final_result;
}
fn fix_update(update: &[i32], map: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut graph = Graph::<i32, (), petgraph::Directed>::new();
    for (before, after) in map.iter() {
        let before_idx = graph.add_node(before.clone());
        for a in after {
            if let Some(a_idx) = graph.node_indices().find(|i| graph[*i] == *a) {
                graph.add_edge(a_idx, before_idx, ());
            } else {
                let a_idx = graph.add_node(a.clone());
                graph.add_edge(a_idx, before_idx, ());
            }
        }
    }

    return Vec::new();
}
fn check_updates(updates: &[i32], map: &HashMap<i32, Vec<i32>>) -> bool {
    for (i, x) in updates.iter().enumerate() {
        if let Some(after) = map.get(&x) {
            for a in after {
                if let Some(a_idx) = updates.iter().position(|n| n == a) {
                    if a_idx < i {
                        return false;
                    }
                }
            }
        }
    }
    return true;
}
#[test]
fn part1_test_input() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let result = part1(input);
    assert_eq!(result, 143);
}
