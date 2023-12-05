use std::{collections::HashMap, ops::Range, str::FromStr};
pub fn part1(input: &String) -> u64 {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let mut map: HashMap<String, SeedMap> = HashMap::new();
    let mut seeds: Vec<u64> = Vec::new();
    for i in 0..sections.len() {
        let section = sections[i];
        if i == 0 {
            seeds = section[7..]
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect();
        } else {
            let seed_map = SeedMap::from_str(section).unwrap();
            map.insert(seed_map.map_from.clone(), seed_map);
        }
    }

    let result = seeds
        .iter()
        .map(|s| {
            let mut current_map = map.get("seed").expect("This wasn't mapped right");
            let mut current_map_from = current_map.map_to.clone();
            let mut src = *s;
            while current_map_from != "location" {
                current_map = map
                    .get(&current_map_from)
                    .expect("This wasn't mapped right");
                current_map_from = current_map.map_to.clone();
                src = current_map.get_dest(src);
            }
            return src;
        })
        .min()
        .unwrap();

    return result;
}

pub fn part2(input: &String) -> u64 {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let mut map: HashMap<String, SeedMap> = HashMap::new();
    let mut seeds: Vec<(u64, u64)> = Vec::new();
    for i in 0..sections.len() {
        let section = sections[i];
        if i == 0 {
            seeds = section[7..]
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|c| (c[0], c[1]))
                .collect::<Vec<_>>();
        } else {
            let seed_map = SeedMap::from_str(section).unwrap();
            map.insert(seed_map.map_from.clone(), seed_map);
        }
    }

    return 0;
}

#[derive(Debug, PartialEq)]
struct SeedMap {
    map_from: String,
    map_to: String,
    ranges: Vec<SeedRange>,
}

#[derive(Debug, PartialEq)]
struct SeedRange {
    source: Range<u64>,
    destination: Range<u64>
}
impl SeedRange {
    fn new(src: u64, dest: u64, length: u64) -> SeedRange {
        return SeedRange {
            source: Range { start: src, end: src+length },
            destination: Range { start: dest, end: dest+length },
        };
    }
}
impl SeedMap {
    fn get_dest_range(&self, src: (u64, u64)) -> Vec<Range<u64>> {
        let possible_map = self
            .ranges
            .iter()
            .filter(|c| src.0 < c.source.end && src.0+src.1 > c.source.start)
            .map(|c| {
                let mut start = src.0;
                let mut end = src.0+src.1;
                c.source.start + c.destination.start - src.0;
                c.source.end + c.destination.end - (src.0 + src.1);
            })
            .collect::<Vec<_>>();

        return Vec::new();
    }
    fn get_dest(&self, src: u64) -> u64 {
        let possible_map = self
            .ranges
            .iter()
            .filter(|c| c.source.contains(&src))
            .collect::<Vec<_>>();
        if possible_map.len() > 0 {
            let range = possible_map.iter().last().unwrap();
            return src - range.source.start + range.destination.start;
        } else {
            return src;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for SeedMap {
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let mut map_from = String::new();
        let mut map_to = String::new();
        let mut gens: Vec<Vec<u64>> = Vec::new();
        for i in 0..lines.len() {
            let line = lines[i];
            if i == 0 {
                map_from = line
                    .replace(" map:", "")
                    .split("-to-")
                    .collect::<Vec<_>>()
                    .first()
                    .unwrap()
                    .to_string();
                map_to = line
                    .replace(" map:", "")
                    .split("-to-")
                    .collect::<Vec<_>>()
                    .last()
                    .unwrap()
                    .to_string();
            } else {
                gens.push(
                    lines[i]
                        .split(" ")
                        .filter(|s| !s.is_empty())
                        .map(|s| s.trim().parse::<u64>().unwrap())
                        .collect::<Vec<_>>(),
                );
            }
        }
        let mut ranges = gens.iter().map(|g| SeedRange::new( g[1], g[0], g[2])).collect::<Vec<_>>();
        Ok(SeedMap {
            map_from: map_from.to_string(),
            map_to: map_to.to_string(),
            ranges,
        })
    }
}
#[test]
fn range_splitting() {
    let expected = vec![Range {start: 0, end: 5}, Range{start: 21, end: 24}, Range{start: 10, end: 15}];
    let seed_range = SeedRange::new(6,21,4);
    let seed_map = SeedMap { map_from: String::from("test"), map_to: String::from("test"), ranges: vec![seed_range]};
    let result = seed_map.get_dest_range((0,15));
    assert_eq!(expected, result);
}
#[test]
fn map_seed_range_from_value () {

    let result = SeedRange::new(0,5,5);
    let expected_source_range = Range { start: 0, end: 5};
    let expected_dest_range = Range { start: 5, end: 10};
    assert_eq!( expected_source_range, result.source);
    assert_eq!( expected_dest_range, result.destination);
}
#[test]
fn map_from_str() {
    let string = String::from(
        "soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15",
    );

    let expected = SeedMap {
        map_from: String::from("soil"),
        map_to: String::from("fertilizer"),
        ranges: vec![SeedRange::new(15, 0, 37), SeedRange::new(52, 37, 2), SeedRange::new(0, 39, 15)],
    };
    let result = SeedMap::from_str(&string).unwrap();
    assert_eq!(expected, result);
}
#[test]
fn get_seed_dest() {
    let map = SeedMap {
        map_from: String::from("soil"),
        map_to: String::from("fertilizer"),
        ranges: vec![SeedRange::new(15, 0, 37), SeedRange::new(52, 37, 2), SeedRange::new(0, 39, 15)],
    };
    let result = map.get_dest(16);
    assert_eq!(1, result);
}
