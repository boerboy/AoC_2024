use std::error::Error;
use std::fs::File;
use itertools::Itertools;

fn read_csv() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let file = File::open("./resources/input_1.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .flexible(true)
        .has_headers(false)
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

fn pop_and_check_list(list: &Vec<i32>, indexes: [usize; 2], error_threshold: i64) -> bool {
    let mut list_0 = list.clone();
    let mut list_1 = list.clone();
    list_0.remove(indexes[0]);
    list_1.remove(indexes[1]);
    check_list(list_0, error_threshold - 1) ||
        check_list(list_1, error_threshold - 1)
}

fn check_list(list: Vec<i32>, error_threshold: i64) -> bool {
    let diffs = list.windows(2).map(|window| window[1] - window[0]).collect_vec();
    let diffs_in_range = diffs.iter().all(|&delta| (1..=3).contains(&delta.abs()));
    let only_increasing = diffs.iter().all(|&delta| delta > 0);
    let only_decreasing = diffs.iter().all(|&delta| delta < 0);
    let pass = diffs_in_range && (only_decreasing || only_increasing);

    if pass {
        return pass;
    }

    if error_threshold != 0 {
        let list_with_index = list.iter().enumerate().collect_vec();
        let diffs = list_with_index.windows(2).map(|window| ([window[0].0, window[1].0], window[1].1 - window[0].1)).collect_vec();
        let diffs_in_range_error = diffs.iter().find(|(_, delta)| !(1..=3).contains(&delta.abs()));
        let is_increasing = diffs.iter().filter(|(_, delta)| *delta > 0).count() as i32 > diffs.iter().filter(|(_, delta)| *delta < 0).count() as i32;
        if is_increasing {
            let only_increasing_err = diffs.iter().find(|(_, delta)| delta < &0);
            diffs_in_range_error.or(only_increasing_err).map(|(indexes, _)| {
                pop_and_check_list(&list, *indexes, error_threshold)
            }).unwrap_or(false)
        } else {
            let only_decreasing_err = diffs.iter().find(|(_, delta)| delta > &0);
            diffs_in_range_error.or(only_decreasing_err).map(|(indexes, _)| {
                pop_and_check_list(&list, *indexes, error_threshold)
            }).unwrap_or(false)
        }
    } else { false }
}

fn main() {
    match read_csv() {
        Ok(value) => {
            let part_1_result = value
                .iter()
                .fold(
                    0,
                    |acc, list| acc + check_list(list.clone(), 0) as i32,
                );
            println!("Part 1 Result: {:?}", part_1_result);

            let part_2_result = value
                .iter()
                .fold(
                    0,
                    |acc, list| acc + check_list(list.clone(), 1) as i32,
                );
            println!("Part 2 Result: {:?}", part_2_result);
        }
        Err(err) => println!("{}", err)
    }
}