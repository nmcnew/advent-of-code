
mod solutions;

use std::fs;
use clap::Parser;
use std::io::ErrorKind;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    
    if args.init {
        fs::create_dir("data")
            .expect("Couldn't create data path");
        let mut i = 1;
        while i <= 25 {
            let path = format!("data/day_{:02}.txt",i);
            let file_result = fs::write(path, "");
            match file_result {
                Ok(file) => file,
                Err(error) => {
                    match error.kind() {
                        ErrorKind::AlreadyExists => println!("{}File Already Exists, skipping", error),
                        _ => panic!("Unexpected error {}", error),
                    }                    
                },
            }
            i += 1;
        }
    }
    else if args.each {
        run_day_01();
        run_day_02();
        run_day_03();
        run_day_04();
        run_day_05();
        run_day_06();
        run_day_07();
        run_day_08();
        run_day_09();
        run_day_10();
        run_day_11();
        run_day_12();
        run_day_13();
        run_day_14();
        run_day_15();
        run_day_16();
        run_day_17();
        run_day_18();
        run_day_19();
        run_day_20();
        run_day_21();
        run_day_22();
        run_day_23();
        run_day_24();
        run_day_25();
        return Ok(()); //Exit early
    }
    else {
        match args.day {
             1 => run_day_01(),
             2 => run_day_02(),
             3 => run_day_03(),
             4 => run_day_04(),
             5 => run_day_05(),
             6 => run_day_06(),
             7 => run_day_07(),
             8 => run_day_08(),
             9 => run_day_09(),
            10 => run_day_10(),
            11 => run_day_11(),
            12 => run_day_12(),
            13 => run_day_13(),
            14 => run_day_14(),
            15 => run_day_15(),
            16 => run_day_16(),
            17 => run_day_17(),
            18 => run_day_18(),
            19 => run_day_19(),
            20 => run_day_20(),
            21 => run_day_21(),
            22 => run_day_22(),
            23 => run_day_23(),
            24 => run_day_24(),
            25 => run_day_25(),
            _ => println!("Did not find a matching day"),
        }
    } 
    Ok(())
}

/// Advent of Code 2023 Runner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day of advent of code to run
    #[arg(short, long, default_value_t=1)]
    day: i32,
    /// Initialize the data files needed for advent of code
    #[arg(short, long, default_value_t=false)]
    init: bool,
    /// Runs every solution
    #[arg(short, long, default_value_t=false)]
    each: bool,
}

fn run_day_01(){
    let values = fs::read_to_string("data/day_01.txt")
        .expect("Day 01 data not initialized");
    let result = solutions::day_01::part1(&values);
    let result2 = solutions::day_01::part2(&values);
    println!("Part 1: {result}, Part 2: {result2}");
}
fn run_day_02(){
    let values = fs::read_to_string("data/day_02.txt")
        .expect("Day 02 data not initialized");
    let result = solutions::day_02::part1(&values);
    let result2 = solutions::day_02::part2(&values);
    println!("Part 1: {result}, Part 2: {result2}");
}
fn run_day_03(){
    let values = fs::read_to_string("data/day_03.txt")
        .expect("Day 03 data not initialized");
    let result = solutions::day_03::part1(&values);
    let result2 = solutions::day_03::part2(&values);
    println!("Part 1: {result}, Part 2: {result2}");
}
fn run_day_04(){
    let values = fs::read_to_string("data/day_04.txt")
        .expect("Day 04 data not initialized");
    let result = solutions::day_04::part1(&values);
    let result2 = solutions::day_04::part2(&values);
    println!("Part 1: {result}, Part 2: {result2}");
}
fn run_day_05(){
    let values = fs::read_to_string("data/day_05.txt")
        .expect("Day 05 data not initialized");
    let result = solutions::day_05::part1(&values);
    let result2 = solutions::day_05::part2(&values);
    println!("Part 1: {result}, Part 2: {result2}");
}
fn run_day_06(){
    let values = fs::read_to_string("data/day_06.txt")
        .expect("Day 06 data not initialized");
    let result = solutions::day_06::part1(&values);
    let result2 = solutions::day_06::part2(&values);
    println!("Part 1: {result}, Part 2: {result2}");
}
fn run_day_07(){
    let values = fs::read_to_string("data/day_07.txt")
        .expect("Day 07 data not initialized");
    let result = solutions::day_07::part1(&values);
    let result2 = solutions::day_07::part2(&values);
    println!("Part 1: {result}, Part 2: {result2}");
}
fn run_day_08(){
    println!("Day 08 is not implented yet");
}
fn run_day_09(){
    println!("Day 09 is not implented yet");
}
fn run_day_10(){
    println!("Day 10 is not implented yet");
}
fn run_day_11(){
    println!("Day 11 is not implented yet");
}
fn run_day_12(){
    println!("Day 12 is not implented yet");
}
fn run_day_13(){
    println!("Day 13 is not implented yet");
}
fn run_day_14(){
    println!("Day 14 is not implented yet");
}
fn run_day_15(){
    println!("Day 15 is not implented yet");
}
fn run_day_16(){
    println!("Day 16 is not implented yet");
}
fn run_day_17(){
    println!("Day 17 is not implented yet");
}
fn run_day_18(){
    println!("Day 18 is not implented yet");
}
fn run_day_19(){
    println!("Day 19 is not implented yet");
}
fn run_day_20(){
    println!("Day 20 is not implented yet");
}
fn run_day_21(){
    println!("Day 21 is not implented yet");
}
fn run_day_22(){
    println!("Day 22 is not implented yet");
}
fn run_day_23(){
    println!("Day 23 is not implented yet");
}
fn run_day_24(){
    println!("Day 24 is not implented yet");
}
fn run_day_25(){
    println!("Day 25 is not implented yet");
}
