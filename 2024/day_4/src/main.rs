use std::error::Error;
use std::fs::File;

const XMAS: &str = "XMAS";

const DIRECTIONS: [(i32, i32); 8] = [
        (0, -1),
        // North East
        (1, -1),
        // North West
        (-1, -1),
        // East
        (1, 0),
        // South East
        (1, 1),
        // South
        (0, 1),
        // South West
        (-1, 1),
        // West
        (-1, 0)
];

fn read_csv() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let file = File::open("./resources/input.csv")?;
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

fn find_char(crossword: &Vec<Vec<char>>, search_char: char) -> Vec<(usize, usize)> {
    crossword.iter().enumerate().flat_map(|(y, value)| {
        value.iter().enumerate().flat_map(move |(x, value)| {
            if *value == search_char {
                Some((x, y))
            } else {
                None
            }
        })
    }).collect()
}

fn fetch_char_at(crossword: Vec<Vec<char>>, row: usize, col: usize) -> String {
    crossword
        .get(row)
        .iter()
        .flat_map(|x| x.get(col))
        .fold(String::new(), |_, x| x.to_string())
}

fn find_xmas(current: usize, coords: (usize, usize), crossword: &Vec<Vec<char>>, direction: Option<(i32, i32)>) -> i64 {
    let (col, row) = coords;
    let base_find = |fn_direction| find_xmas(current, coords, crossword, fn_direction);
    if direction.is_none() {
        DIRECTIONS.iter().map(|direction|  base_find(Some(*direction))).sum()
    } else {
        direction.iter().map(|(col_dir, row_dir)|{
            if current == 3 {
                1i64
            } else {
                let (new_col, new_row) = ((col as i32 + col_dir) as usize, (row as i32 + row_dir) as usize);
                let actual = fetch_char_at(crossword.clone(), new_row, new_col);
                let _actual_str = actual.as_str();
                let looking_for = XMAS.get(current + 1 ..(current + 2) ).unwrap_or("");
                if actual == looking_for {
                    find_xmas(current + 1, (new_col, new_row), crossword, direction)
                } else {
                    0i64
                }
            }
        }).sum()
    }

}

fn find_ms(m: String, s: String) -> bool {
    m == "M" && s == "S"
}

fn find_symmetries(bottom: String, top: String) -> i64 {
    let opposite = if find_ms(bottom.clone(), top.clone()) {
        1
    } else {
        0
    };

    let symmetrical = if find_ms(top.clone(), bottom.clone()) {
        1
    } else {
        0
    };
    opposite + symmetrical
}

fn find_mas( coords: (usize, usize), crossword: &Vec<Vec<char>>) -> i64 {
    let (col, row) = coords;
    let north_col = (col as i32 - 1) as usize;
    let south_col =  (col as i32 + 1) as usize;
    let east_row =  (row as i32 + 1) as usize;
    let west_row =  (row as i32 - 1) as usize;
    let top_left = fetch_char_at(crossword.clone(), west_row, north_col);
    let top_right = fetch_char_at(crossword.clone(), east_row, north_col);
    let bottom_left = fetch_char_at(crossword.clone(), west_row, south_col);
    let bottom_right = fetch_char_at(crossword.clone(), east_row, south_col);
    //Written forwards
    if top_left == "M" && find_ms(top_left.clone(), bottom_right.clone()) {
        find_symmetries(bottom_left, top_right)
    } else if bottom_right == "M"  && find_ms(bottom_right.clone(), top_left.clone()){
        find_symmetries(bottom_left, top_right)
    } else {
        0
    }
}

fn main() {
    match read_csv() {
        Ok(crossword) => {
            let xs = find_char(&crossword, 'X');
            let result: i64 = xs.iter().map(|coords| find_xmas(0, *coords, &crossword, None)).sum();
            println!("Part 1: {:?}", result);

            let az = find_char(&crossword, 'A');
            let result: i64 = az.iter().map(|coords| find_mas(*coords, &crossword)).sum();
            println!("Part 2: {:?}", result);
        }
        Err(err) => println!("{}", err)
    }


}
