use std::collections::HashMap;
use std::fs;
use itertools::Itertools;
use regex::Regex;

fn part_1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let num1 = caps.get(1).unwrap().as_str().parse::<i64>().ok().expect("Failed to parse number 1");
            let num2 = caps.get(2).unwrap().as_str().parse::<i64>().ok().expect("Failed to parse number 2");
            (num1, num2)
        })
        .collect_vec()
        .iter()
        .fold(0i64, |acc, (x, y)| acc + (x * y))
}

fn build_index_list(re: Regex, input_str: &str) -> Vec<usize> {
    re
        .captures_iter(input_str)
        .map(|caps| {
            let cap = caps.get(0).unwrap();
            cap.start()
        }).collect_vec()
}

fn part_2(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let dont_sections = Regex::new(r"don't\(\)").unwrap();
    let do_sections = Regex::new(r"do\(\)").unwrap();
    let dont_locations = build_index_list(dont_sections, input);
    let do_locations = build_index_list(do_sections, input);
    let multiplications: HashMap<usize, i64> = re
        .captures_iter(input)
        .map(|caps| {
            let start = caps.get(1).unwrap().start();
            let num1 = caps.get(1).unwrap().as_str().parse::<i64>().ok().expect("Failed to parse number 1");
            let num2 = caps.get(2).unwrap().as_str().parse::<i64>().ok().expect("Failed to parse number 2");
            (start, num1 * num2)
        })
        .collect();
    input.chars().enumerate().into_iter().fold((0i64, true), |(sum, active), (index, _)| {
        let currently_active = active && !dont_locations.contains(&index) || do_locations.contains(&index);
        if currently_active {
            (sum + multiplications.get(&index).unwrap_or(&0i64), currently_active)
        } else {
            (sum, currently_active)
        }
    }).0
}


pub fn main() {
    match fs::read_to_string("./resources/input_1.txt") {
        Ok(input) => {
            println!("{:?}", part_1(&input));
            println!("{:?}", part_2(&input))
        }
        Err(err) => println!("{}", err)
    }
}