use std::error::Error;
use std::fs::File;
use std::time::Instant;
use rayon::prelude::*;

const CARDINALS: [(i32, i32); 4] = [
    // North
    (0, -1),
    // East
    (1, 0),
    // South
    (0, 1),
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

fn fetch_char_at(crossword: &Vec<Vec<char>>, col: usize, row: usize) -> String {
    crossword
        .get(row)
        .iter()
        .flat_map(|x| x.get(col))
        .fold(String::new(), |_, x| x.to_string())
}

fn walk(
    crossword: &Vec<Vec<char>>,
    walked: &mut Vec<Vec<char>>,
    current_coords: (usize, usize),
    direction_index: usize
) {
    let direction_index = if direction_index == 4 {0} else { direction_index };
    let (col_dir, row_dir) = CARDINALS[direction_index];
    let walked_coords = ((current_coords.0 as i32 + col_dir) as usize, (current_coords.1 as i32 + row_dir) as usize);
    let walked_char = fetch_char_at(crossword, walked_coords.0, walked_coords.1);
    if walked_char == "#"  {
        walk(crossword, walked, current_coords, direction_index+1usize);
    } else if walked_char != "" {
        walked[walked_coords.0][walked_coords.1] = 'X';
        walk(crossword, walked, walked_coords, direction_index);
    }
}

fn draw_path(coords: (usize, usize), crossword: &Vec<Vec<char>>, direction_index: usize) -> Vec<Vec<char>> {
    let mut walked = crossword.clone();
    walk(crossword, &mut walked, coords, direction_index);
    walked
}


fn find_loop(
    crossword: &Vec<Vec<char>>,
    mut current_coords: (usize, usize),
    mut direction_index: usize,
    updated_coords: (usize, usize),
    mut depth: i32
) -> i32 {

    loop {
        direction_index = if direction_index == 4 {0} else { direction_index };
        let (col_dir, row_dir) = CARDINALS[direction_index];
        let walked_coords = ((current_coords.0 as i32 + col_dir) as usize, (current_coords.1 as i32 + row_dir) as usize);
        let walked_char =  if updated_coords.0 == walked_coords.0 && updated_coords.1 == walked_coords.1 {
            String::from("#")
        } else {
            fetch_char_at(crossword, walked_coords.0, walked_coords.1)
        };

        if walked_char == "#"  {
            direction_index+=1usize;
            depth+= 1;
        } else if walked_char != "" {
            current_coords = walked_coords;
        } else if walked_char == "" {
            break
        }
        // lazy recursion check - in theory it is number of '#' chars and a margin of some level
        if depth > 150 {
            return  1
        };
    }

    0 // No loop found within the given constraints
}


fn find_loops(coords: (usize, usize), crossword: &Vec<Vec<char>>, direction_index: usize) -> i32 {
    crossword
        .par_iter()
        .enumerate()
        .flat_map(|(y_index, rows) | rows.par_iter().enumerate().map(move |(x_index, _)| {
            if(x_index == coords.0 && y_index == coords.1)  || fetch_char_at(crossword, x_index, y_index) == "#" {
                0
            } else {
                find_loop(crossword, coords, direction_index, (x_index, y_index), 0)
            }
        })).sum()
}

fn main() {
    match read_csv() {
        Ok(crossword) => {
            let guards = find_char(&crossword, '^');
            let result = guards
                .iter()
                .flat_map(|coords| draw_path(*coords, &crossword, 0))
                .flat_map(|x|x)
                .filter(|x| String::from(*x) == "X")
                .count();
            println!("Part 1: {:?}", result);
            let start = Instant::now();
            // Code to measure
            let result: i32 = guards
                .iter()
                .map(|coords| find_loops(*coords, &crossword, 0))
                .sum();
            println!("Part 2: {:?}", result);
            let duration = start.elapsed();
            println!("Time taken: {:?}", duration)
        }
        Err(err) => println!("{}", err)
    }
}
