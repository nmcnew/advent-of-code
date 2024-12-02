use std::collections::HashMap;

pub fn part1(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();
    let mut instructions = Vec::new();
    let mut nodes = HashMap::new();
    for i in 0..lines.len() {
        let line = lines[i];
        if i == 0 {
            instructions = line.chars().map(|c| Direction::from_char(c)).collect();
        } else if i > 1 {
            let new_node = Node::from_str(line);
            let key = new_node.key.to_string();
            nodes.insert(key, new_node);
        }
    }
    let map = Map::new(instructions, nodes);
    let path = map.navigate("AAA", "ZZZ");
    return path.len() as i32;
}

pub fn part2(input: &str) -> i32 {
    return 0;
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Unexepcted Character"),
        }
    }
}
#[derive(Debug)]
struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

impl Map {
    fn new(instructions: Vec<Direction>, nodes: HashMap<String, Node>) -> Self {
        Self {
            instructions,
            nodes,
        }
    }
    fn navigate(&self, start: &str, end: &str) -> Vec<Direction> {
        let current_node = self.nodes.get(start).expect("Invalid start node");
        return self.navigate_internal(current_node, end, 0);
    }
    fn navigate_internal(&self, current: &Node, end: &str, mut step: usize) -> Vec<Direction> {
        if current.key == end {
            return vec![];
        }
        if step >= self.instructions.len() {
            step = 0;
        }
        let move_direction = self.instructions[step];

        let next_node = match move_direction {
            Direction::Left => self.nodes.get(&current.left_key).unwrap(),
            Direction::Right => self.nodes.get(&current.right_key).unwrap(),
        };
        if current == next_node {
            panic!("Caught in a loop");
        }
        let mut result = self.navigate_internal(next_node, end, step + 1);
        result.push(move_direction);
        return result;
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Node {
    key: String,
    left_key: String,
    right_key: String,
}

impl Node {
    fn from_str(input: &str) -> Node {
        let mut parts = input.split(" = ");
        let key = parts.next().unwrap().to_string();
        let mut left_and_right = parts
            .next()
            .unwrap()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ");
        let left_key = left_and_right.next().unwrap().to_string();
        let right_key = left_and_right.next().unwrap().to_string();
        Node {
            key,
            left_key,
            right_key,
        }
    }
}

#[test]
fn parse_node() {
    let input = "AAA = (BBB, CCC)";
    let expected = Node {
        key: String::from("AAA"),
        left_key: String::from("BBB"),
        right_key: String::from("CCC"),
    };
    let result = Node::from_str(input);

    assert_eq!(expected, result);
}
#[test]
fn part1_simple() {
    let input = String::from(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
    );

    let actual = part1(&input);

    assert_eq!(2, actual);
}
