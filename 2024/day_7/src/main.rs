use std::error::Error;
use std::fs::File;
use std::i64;
use itertools::Itertools;

fn read_csv() -> Result<Vec<(i64,Vec<i64>)>, Box<dyn Error>> {
    let file = File::open("./resources/input.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .flexible(true)
        .from_reader(file);

    let results = rdr.records()
        .flat_map(|x|x)
        .map(|x| {
            let mut row = x.iter().map(|x| x.replace(":","").parse::<i64>().expect( "Failed to parse string as i32")).collect_vec();
            let head = *row.get(0usize).unwrap_or(&0i64);
            row.remove(0);
            (head, row)
        })
        .collect_vec();

    Ok(results)
}

fn apply_operator(x:i64, y:i64, operator_index: i32) -> i64 {
    match operator_index {
        0 => x + y,
        1 => x * y,
        2 => (x.to_string() +  y.to_string().as_str()).parse().expect("Successful string to integer parsing"),
        _ => panic!("Unhandled case")
    }
}

fn calibrate(input: &(i64,Vec<i64>), calibration_range:i32) -> i64 {
    let (calibration_value, values) = input;
    fn inner(calibration_value: i64, values: &Vec<i64>, calibration_range: i32, current_index: usize, total: i64) -> bool {
        let current_value = values.get(current_index);
        if current_value.is_none() {
            total == calibration_value
        } else {
            (0..calibration_range).position(|x|{
                inner(calibration_value, values, calibration_range,current_index+1usize, apply_operator(total, *current_value.unwrap(), x))
            }).is_some()
        }
    }
    if inner(*calibration_value, values, calibration_range, 0, 0) {*calibration_value} else {0}
}

fn main() {
    let input = read_csv().expect("Should read and parse csv");
    let part_1: i64 = input
        .iter()
        .map(|x| calibrate(x, 2))
        .sum();

    println!("Part 1: {}", part_1);


    let part_2: i64 = input
        .iter()
        .map(|x| calibrate(x, 3))
        .sum();

    println!("Part 2: {}", part_2);

}
