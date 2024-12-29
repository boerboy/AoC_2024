use common::reader::read_single_line_delimited;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

fn create_cache(blinks: i64) -> HashMap<i64, HashMap<i64, i64>> {
    let mut cache: HashMap<i64, HashMap<i64, i64>> = HashMap::new();

    (0..10).for_each(|x| {
        let mut cache_inner: HashMap<i64, i64> = HashMap::new();
        (1..blinks).for_each(|depth| {
            cache_inner.insert(depth, apply_rules(&vec![x], 0, depth, &HashMap::new()));
        });
        cache.insert(x, cache_inner);
    });
    cache
}

fn apply_rule(value: i64) -> Vec<i64> {
    match value {
        x if x == 0 => vec![1],
        x if x.to_string().len() % 2 == 0 => {
            let x_str = x.to_string();
            let (head, tail) = x_str.split_at(x_str.len() / 2);
            vec![
                head.parse::<i64>()
                    .expect("Successful string parse and trim"),
                tail.parse::<i64>()
                    .expect("Successful string parse and trim"),
            ]
        }
        x => vec![x * 2024],
    }
}

fn apply_rules(
    values: &Vec<i64>,
    current_depth: i64,
    maximum_depth: i64,
    cache: &HashMap<i64, HashMap<i64, i64>>,
) -> i64 {
    if current_depth == maximum_depth {
        values.iter().count() as i64
    } else {
        values
            .par_iter()
            .map(|x| {
                let cached: Option<&i64> = cache.get(&x).iter().flat_map(|x|x.get(&(maximum_depth - current_depth))).next();
                if cached.is_some() {
                    *cached.unwrap()
                } else {
                    apply_rules(&apply_rule(*x), current_depth + 1, maximum_depth, cache)
                }
            })
            .sum()
    }
}

fn apply_rules_with_cache(input: &Vec<i64>, blinks: i64) -> i64 {
    let cache = create_cache(blinks/2);
    input
        .par_iter()
        .map(|x| apply_rules(&vec![*x], 0, blinks, &cache))
        .sum()
}

fn main() {
    let input: Vec<i64> =
        read_single_line_delimited("./resources/input.csv", ' ').expect("Successful input read");
    let part_1 = apply_rules_with_cache(&input, 25);
    println!("Part 1: {:?}", part_1);
    let part_2 = apply_rules_with_cache(&input, 75);
    println!("Part 2: {:?}", part_2)
}
