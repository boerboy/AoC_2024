use itertools::Itertools;
use common::coords::Coords;
use common::grid::Grid;
use common::reader::read_grid_default;


fn traverse_path(input: &Grid<i32>, coords: &Coords) -> i32 {
    fn inner(input: &Grid<i32>, coords: Coords, search_val: i32) -> i32 {
        Coords::CARDINALS
            .iter()
            .fold(0, |acc, Coords {x: cardinal_col, y: cardinal_row }| {
                let new_row = coords.y + cardinal_row;
                let new_col = coords.x + cardinal_col;
                let x = *input
                    .inner
                    .get(new_row as usize)
                    .map(|x|x.get(new_col as usize))
                    .flatten()
                    .unwrap_or(&-1);

                if x == 9 && search_val == 9 {
                    println!("head at {:?}, {:?}, acc: {:?}", new_col, new_row, acc);
                    1
                } else if x == search_val {
                    acc + inner(input, Coords{x: new_col, y: new_row }, search_val + 1)
                } else {
                    acc
                }
            })
    }
    inner(input, *coords,1)
}

fn traverse_paths(input: &Grid<i32>, starts: &Vec<Coords>) -> i32 {
    starts
        .iter()
        .fold(0, |acc, coords|{
            acc + traverse_path(input, coords)
        })

}

fn main() {
    let input = &read_grid_default("./resources/test.csv").expect("Successful input read");
    let starts = input.find(0);
    let part_1 = traverse_paths(input, &starts);
    println!("{:?}",part_1);
}
