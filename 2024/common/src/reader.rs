use std::error::Error;
use std::fs;
use std::fs::File;
use std::str::FromStr;
use itertools::Itertools;
use serde::de::DeserializeOwned;
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

pub fn read_single_line_delimited<T>(file: &str, delimiter: char) -> Result<Vec<T>, std::io::Error>
where T: FromStr {
    let data = fs::read_to_string(file)?;
    let parsed = data.split(delimiter).flat_map(|x|x.parse::<T>()).collect_vec();
    Ok(parsed)
}

pub fn read_csv<T>(file: &str, delimiter: u8) -> Result<Vec<T>, Box<dyn Error>>
where T: DeserializeOwned {
    let file = File::open(file)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .from_reader(file);

    let result = rdr
        .deserialize()
        .flat_map(|x| x)
        .collect();

    Ok(result)
}