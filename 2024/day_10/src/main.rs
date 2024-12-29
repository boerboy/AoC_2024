use std::collections::HashSet;
use common::coords::Coords;
use common::grid::Grid;
use common::reader::read_grid_default;

fn traverse_path<'a>(input: &Grid<i32>, coords: &Coords) -> Vec<Coords> {
    fn inner(input: &Grid<i32>, coords: Coords, search_val: i32) -> Vec<Coords> {
        Coords::CARDINALS
            .iter()
            .fold(vec![], |mut acc, Coords {x: cardinal_col, y: cardinal_row }| {
                let new_row = coords.y + cardinal_row;
                let new_col = coords.x + cardinal_col;
                let x = *input
                    .inner
                    .get(new_row as usize)
                    .map(|x|x.get(new_col as usize))
                    .flatten()
                    .unwrap_or(&-1);

                if x == 9 && search_val == 9 {
                    acc.push(Coords{x: new_col, y: new_row });
                    acc
                } else if x == search_val {
                    acc.extend(inner(input, Coords{x: new_col, y: new_row }, search_val + 1));
                    acc
                } else {
                    acc
                }
            })
    }
    inner(input, *coords,1)
}

fn traverse_paths_distinct_heads(input: &Grid<i32>, starts: &Vec<Coords>) -> i32 {
    starts
        .iter()
        .fold(0, |acc, coords|{
            let heads: HashSet<Coords> = HashSet::from_iter(traverse_path(input, coords));
            acc + heads.iter().count() as i32
        })
}

fn traverse_paths_distinct_paths(input: &Grid<i32>, starts: &Vec<Coords>) -> i32 {
    starts
        .iter()
        .fold(0, |acc, coords|{
            let heads = traverse_path(input, coords);
            acc + heads.iter().count() as i32
        })
}


fn main() {
    let input = &read_grid_default("./resources/input.csv").expect("Successful input read");
    let starts = input.find(0);
    let part_1 = traverse_paths_distinct_heads(input, &starts);
    println!("Part 1: {:?}",part_1);
    let part_2 = traverse_paths_distinct_paths(input, &starts);
    println!("Part 2: {:?}",part_2);
}
