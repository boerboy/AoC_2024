use std::error::Error;
use std::fs::File;
use std::str::FromStr;
use itertools::Itertools;
use crate::grid::Grid;

pub fn read_grid<T>(file: &str, delimiter: u8) ->  Result<Grid<T>, Box<dyn Error>>
where T: FromStr {
    let file = File::open(file)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .from_reader(file);

    let result: Vec<Vec<T>> = rdr.records()
        .flat_map(|x| x)
        .map(|x| {
            x.iter().flat_map(|x|x.chars().flat_map(|y| y.to_string().parse::<T>())).collect()
        }).collect_vec();

    Ok(Grid{inner: result})
}

pub fn read_grid_default<T>(file: &str) -> Result<Grid<T>, Box<dyn Error>>
where T: FromStr {
    read_grid(file, b' ')
}