/**
Ran out of dedicated time and solution borrowed from https://github.com/Krever/advent-of-code/blob/ec23a69d397b807061609619a54afcc0fcbe8c2a/2024/src/day5.rs
#TODO Come back when and re-implement using own methods
**/

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use itertools::Itertools;

fn read_rules_csv() -> Result<HashMap<i32, Vec<i32>>, Box<dyn Error>> {
    let file = File::open("./resources/rules.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_reader(file);

    let result_map = rdr.records()
        .flat_map(|x|x)
        .map(|x| {
            (x[0].parse::<i32>().expect( "Failed to parse string as i32"), x[1].parse::<i32>().expect( "Failed to parse string as i32"))
        })
        .into_group_map();

    Ok(result_map)
}

fn read_input_csv() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let file = File::open("./resources/input.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(file);

    let mut result_list: Vec<Vec<i32>> = vec![];

    for result in rdr.records() {
        let record = result?
            .iter()
            .flat_map(|x| { x.parse::<i32>().map_err(|_| "Failed to parse string as i32") })
            .collect_vec();
        result_list.push(record);
    }

    Ok(result_list)
}

fn is_correct(input: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    let positions: HashMap<&i32, usize> = input
        .iter()
        .enumerate()
        .map(|(idx, value)| (value, idx))
        .collect();
    input.iter().all(|page| {
        let page_pos = positions.get(page).unwrap();
        let empty = Vec::new();
        let applicable_rules = rules.get(&page).unwrap_or(&empty);
        applicable_rules
            .iter()
            .all(|constraint| positions.get(constraint).unwrap_or(&usize::MAX) > page_pos)
    })
}

fn sort(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut sorted: Vec<i32> = Vec::new();
    let mut remaining: HashSet<&i32> = update.iter().collect();

    while sorted.len() != update.len() {
        let without_constraints: Vec<i32> = remaining
            .iter()
            .filter(|&&page| {
                let empty = Vec::new();
                let applicable_rules = rules.get(&page).unwrap_or(&empty);
                applicable_rules
                    .iter()
                    .all(|&required| sorted.contains(&required) || !update.contains(&required))
            })
            .map(|&&page| page)
            .collect();

        if without_constraints.is_empty() {
            panic!("A cycle was detected or unsatisfiable constraints exist!");
        }

        sorted.extend(&without_constraints);
        for page in without_constraints {
            remaining.remove(&page);
        }
    }
    sorted
}


fn part1(input: &Vec<Vec<i32>>, rules: &HashMap<i32, Vec<i32>>) {
    let sum: i32 = input
        .iter()
        .filter(|&x| is_correct(x, &rules))
        .map(|x| x[x.len() / 2])
        .sum();
    println!("Part 1: {}", sum);
}

fn part2(input: &Vec<Vec<i32>>, rules: &HashMap<i32, Vec<i32>>) {
    let sum: i32 = input
        .iter()
        .filter(|&x| !is_correct(x, &rules))
        .map(|x| sort(x, rules))
        .map(|x| x[x.len() / 2])
        .sum();
    println!("Part 2: {}", sum);
}


fn handle_parts() -> Result<(), Box<dyn Error>> {
    let rules = read_rules_csv()?;
    let input = read_input_csv()?;
    part1(&input.clone(), &rules.clone());
    Ok(part2(&input.clone(), &rules.clone()))
}

fn main() {
    let _ = handle_parts().expect("Error during exercise");
}
