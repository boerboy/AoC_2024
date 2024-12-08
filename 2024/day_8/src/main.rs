use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use itertools::Itertools;

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

fn find_diffs(coords_1: (i32, i32), coords_2: (i32, i32)) -> (i32, i32) {
    let (x_1, y_1) = coords_1;
    let (x_2, y_2) = coords_2;
    (x_2 - x_1, y_2 - y_1)
}

fn find_y_intersect(coords_1: (i32, i32), m: f32) -> f32 {
    coords_1.1 as f32 - (m * coords_1.0 as f32)
}

fn build_graph_function(coords_1: (i32, i32), coords_2: (i32, i32)) -> impl Fn(i32) -> f32 {
    let diffs = find_diffs(coords_1, coords_2);
    let m = diffs.1 as f32 / diffs.0 as f32;
    let c = find_y_intersect(coords_1, m);
    move |x| m * x as f32 + c
}


fn is_within_map(col: i32, col_count: usize, row: i32, row_count: usize) -> bool {
    col < col_count as i32 && row < row_count as i32 && col >= 0 && row >= 0
}

fn fetch_next_coords(coords: Option<(i32, i32)>, graph: impl Fn(i32) -> f32) -> Option<(i32, i32)> {
    coords.iter().flat_map(|x| {
        let y = graph(x.0 + 1);
        if y.fract() == 0f32 {
            Some((x.0 + 1, y as i32))
        } else {
            None
        }
    }).next()
}

fn find_antinodes(antenna_map: &Vec<Vec<char>>, mut resonance: bool) -> Vec<(i32, i32)> {
    let (row_count, col_count) = (antenna_map.iter().count(), antenna_map.get(0).unwrap_or(&Vec::<char>::new()).iter().count());

    HashSet::<(i32, i32)>::from_iter(
        antenna_map
            .iter()
            .enumerate()
            .flat_map(|(y_index, y)| y.iter().enumerate().map(move |(x_index, x)| (x, (x_index as i32, y_index as i32))))
            .filter(|x| x.0.to_string() != ".")
            .into_group_map()
            .iter()
            .flat_map(|(antenna, coords_list)| {
                coords_list
                    .iter()
                    .flat_map(move |coords_1| {
                        coords_list
                            .iter()
                            .flat_map(move |coords_2| {
                                let diffs = find_diffs(*coords_1, *coords_2);
                                let mut antinode_list = Vec::<(i32, i32)>::new();
                                if diffs.0 == 0 && diffs.1 == 0 {
                                    vec![]
                                } else {
                                    let graph = build_graph_function(*coords_1, *coords_2);
                                    let mut inner_coords_1: Option<(i32, i32)> = if resonance {
                                        fetch_next_coords(Some(*coords_2), &graph)
                                    } else {
                                        Some((coords_1.0 - diffs.0, coords_1.1 - diffs.1))
                                    };
                                    let mut inner_coords_2: Option<(i32, i32)> = if resonance {
                                        fetch_next_coords(Some(*coords_1), &graph)
                                    } else {
                                        Some((coords_2.0 + diffs.0, coords_2.1 + diffs.1))
                                    };
                                    loop {
                                        let other: Vec<(i32, i32)> = vec![
                                            inner_coords_1,
                                            inner_coords_2
                                        ].iter().flatten().copied().collect();
                                        if other.iter().all(|(x, y)| !is_within_map(*x, col_count, *y, row_count)) {
                                            resonance = false
                                        }
                                        antinode_list.extend(other);

                                        if !resonance { break; }
                                        inner_coords_1 = fetch_next_coords(inner_coords_1, &graph);
                                        inner_coords_2 = fetch_next_coords(inner_coords_2, &graph);
                                    }

                                    antinode_list
                                }
                            })
                    })
            })
    )
        .iter()
        .filter(|(x, y)| is_within_map(*x, col_count, *y, row_count))
        .map(|x| *x)
        .collect_vec()
}


fn main() {
    let input = read_csv().expect("Successful input read");
    let part_1 = find_antinodes(&input, false).iter().count();
    println!("Part 1: {:?}", part_1);
    let part_1 = find_antinodes(&input, true).iter().count();
    println!("Part 2: {:?}", part_1);
}
