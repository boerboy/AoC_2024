use std::error::Error;
use std::fs::File;

fn read_csv() -> Result<(Vec<i64>, Vec<i64>), Box<dyn Error>> {
    let file = File::open("./resources/input_1.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_reader(file);

    let mut result_list: (Vec<i64>, Vec<i64>) = (vec![], vec![]);

    for result in rdr.records() {
        let record = result?;
        let x: i64 = record[0].parse().map_err(|_| "Failed to parse string as i64")?;
        let y: i64 = record[1].parse().map_err(|_| "Failed to parse string as i64")?;
        result_list.0.push(x);
        result_list.1.push(y);
    }

    Ok(result_list)
}

fn find_distance_between_smallest_at_position(position: usize, location_list: Vec<i64>) -> i64 {
    let mut location_list_tmp: Vec<i64> = location_list.to_vec();
    location_list_tmp.sort_by(|a, b| a.cmp(b));
    location_list_tmp[position]
}

fn sum_distances(acc: i64, position: usize, list_1: Vec<i64>, list_2: Vec<i64>) -> i64 {
    let dist_1 = find_distance_between_smallest_at_position(position, list_1);
    let dist_2 = find_distance_between_smallest_at_position(position, list_2);
    acc + (dist_1 - dist_2).abs()
}

fn count_elements(acc: i64, list: Vec<i64>, value: &i64) -> Result<i64, Box<dyn Error>> {
   let size = i64::try_from(list.iter().filter(|&&x| x == *value).count())?;
    Ok(acc + value * size)
}

fn main() {
    match read_csv() {
        Ok(result) => {
            let part_1 = result.0.iter().enumerate().fold(0, |acc, (index, _value)| {
                sum_distances(acc, index, result.0.clone(), result.1.clone())
            });

            let part_2 =  result.0.iter().fold(0, |acc, value| {
               match count_elements(acc, result.1.clone(), value) {
                   Ok(value) => value,
                   _ => -1
               }
            });
            println!("Part 1 Result: {}", part_1);
            println!("Part 2 Result: {}", part_2);

        },
        Err(err) => println!("{}", err)
    }
}
