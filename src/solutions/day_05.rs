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
    let mut seeds: Vec<(u64, u64)> = Vec::new();
    let mut plots = Vec::new();
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
            plots.push(SeedMap::from_str(section).unwrap());
        }
    }
    let super_map = SuperSeedMap::from_vec(plots);

    seeds.sort_by(|s1, s2| s1.0.cmp(&s2.0));

    //build Vec<Vec<u64>> where the seeds could start at
    for i in 0..seeds.len() - 1 {
        if i == seeds.len() - 1 {
            break;
        }
        let mut seed1 = seeds[i];
        let seed2 = seeds[i + 1];

        if seed1.0 + seed1.1 > seed2.0 {
            seed1.1 = seed1.0 + seed2.0 - seed1.0 - 1;
        }
    }

    let mut lowest = u64::MAX;
    let mut lowest_src = 0;
    for seed in seeds {
        for i in (seed.0..seed.0 + seed.1).step_by(1000) {
            let loc = super_map.get_location(i);
            println!("{i} {loc}");
            if loc < lowest {
                lowest = loc;
                lowest_src = i;
            }
        }
    }

    for i in lowest_src..lowest_src + 1000 {
        let loc = super_map.get_location(i);
        println!("{i} {loc}");
        if loc < lowest {
            lowest = loc;
        }
    }
    return lowest;
}

#[derive(Debug, PartialEq)]
struct SuperSeedMap {
    maps: HashMap<String, SeedMap>,
}

impl SuperSeedMap {
    fn from_vec(input: Vec<SeedMap>) -> SuperSeedMap {
        let mut seed_maps = HashMap::new();
        for map in input {
            seed_maps.insert(map.map_from.clone(), map);
        }

        return SuperSeedMap { maps: seed_maps };
    }
    fn get_location(&self, s: u64) -> u64 {
        let mut current_map = self.maps.get("seed").expect("This wasn't mapped right");
        let mut current_map_from = current_map.map_to.clone();
        let mut src = s;
        while current_map_from != "location" {
            current_map = self
                .maps
                .get(&current_map_from)
                .expect("This wasn't mapped right");
            current_map_from = current_map.map_to.clone();
            src = current_map.get_dest(src);
        }
        return src;
    }
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
    destination: Range<u64>,
}

impl SeedRange {
    fn new(src: u64, dest: u64, length: u64) -> SeedRange {
        return SeedRange {
            source: Range {
                start: src,
                end: src + length,
            },
            destination: Range {
                start: dest,
                end: dest + length,
            },
        };
    }
}

impl SeedMap {
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
        let ranges = gens
            .iter()
            .map(|g| SeedRange::new(g[1], g[0], g[2]))
            .collect::<Vec<_>>();
        Ok(SeedMap {
            map_from: map_from.to_string(),
            map_to: map_to.to_string(),
            ranges,
        })
    }
}

#[test]
fn map_seed_range_from_value() {
    let result = SeedRange::new(0, 5, 5);
    let expected_source_range = Range { start: 0, end: 5 };
    let expected_dest_range = Range { start: 5, end: 10 };
    assert_eq!(expected_source_range, result.source);
    assert_eq!(expected_dest_range, result.destination);
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
        ranges: vec![
            SeedRange::new(15, 0, 37),
            SeedRange::new(52, 37, 2),
            SeedRange::new(0, 39, 15),
        ],
    };
    let result = SeedMap::from_str(&string).unwrap();
    assert_eq!(expected, result);
}
#[test]
fn get_seed_dest() {
    let map = SeedMap {
        map_from: String::from("soil"),
        map_to: String::from("fertilizer"),
        ranges: vec![
            SeedRange::new(15, 0, 37),
            SeedRange::new(52, 37, 2),
            SeedRange::new(0, 39, 15),
        ],
    };
    let result = map.get_dest(16);
    assert_eq!(1, result);
}
