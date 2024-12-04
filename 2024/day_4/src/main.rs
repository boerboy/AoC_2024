use std::error::Error;
use std::fs::File;
use itertools::Itertools;

const XMAS: &str = "XMAS";

fn read_csv() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let file = File::open("./resources/test.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_reader(file);

    let mut result_list: Vec<Vec<char>> = vec![];

    for result in rdr.records() {
        let record = result?
            .iter()
            .flat_map(|x| x.chars())
            .collect();
        result_list.push(record);
    }

    Ok(result_list)
}

fn find_x(crossword: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    crossword.iter().enumerate().flat_map(|(y, value)| {
        value.iter().enumerate().flat_map(move |(x, value)| {
            if *value == 'X' {
                Some((x, y))
            } else {
                None
            }
        })
    }).collect()
}

fn find_xmas(current: usize, coords: (usize, usize), crossword: &Vec<Vec<char>>, direction: Option<(i32, i32)>) -> i64 {
    let (col, row) = coords;
    let base_find = |fn_direction| find_xmas(current, coords, crossword, fn_direction);
    if direction.is_none() {
        // North
        base_find(Some((0, 1)))
        // North East
        + base_find(Some((1, 1)))
        // East
        + base_find(Some((1, 0)))
        // South East
        + base_find(Some((1, -1)))
        // South
        + base_find(Some((0, -1)))
        // South West
        + base_find(Some((-1, -1)))
        // West
        + base_find(Some((-1, 0)))
    } else {
        direction.iter().map(|(col_dir, row_dir)|{
            if current == 3 {
                1i64
            } else {
                let (new_col, new_row) = ((row as i32 + row_dir) as usize, (col as i32 +col_dir) as usize);
                let actual = crossword
                        .get(new_row)
                        .iter()
                        .flat_map(|x| x.get(new_col))
                        .fold(String::new(), |_, x| x.to_string());
                let _actual_str = actual.as_str();
                let looking_for = XMAS.get(current + 1 ..(current + 2) ).unwrap_or("");
                if actual == looking_for {
                    find_xmas(current + 1, (new_row, new_col), crossword, direction)
                } else {
                    0i64
                }
            }
        }).sum()
    }

}

fn main() {
    match read_csv() {
        Ok(crossword) => {
            let xs = find_x(&crossword);
            let result: Vec<i64> = xs.iter().map(|coords| find_xmas(0, *coords, &crossword, None)).collect();
            println!("{:?}", result);
        }
        Err(err) => println!("{}", err)
    }


}
