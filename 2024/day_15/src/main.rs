use common::coords::Coords;
use common::grid::Grid;
use common::reader::{read_csv, read_grid_default};

struct WarehouseDetails { grid: Grid<char>, current_position: Coords }

fn walk(grid: &Grid<char>, instructions: Vec<Coords>) -> i32 {
    let start = grid.find_one('@').expect("Start exists");
    instructions
        .iter()
        .fold(
            WarehouseDetails {grid: grid.clone(), current_position: start } ,
            |WarehouseDetails {grid: mut grid, mut current_position}, instruction|{
                let potential_coords = current_position.add(*instruction);
                let potential_char = grid.fetch_at(potential_coords).expect("Char to exist");
                if *potential_char == '#' {
                    WarehouseDetails { grid, current_position}
                } else if *potential_char == '.' {
                    WarehouseDetails { grid, current_position: potential_coords}
                } else if *potential_char == 'O' {
                    fn attempt_move(mut inner_grid:  Grid<char>, current: Coords, direction: Coords) -> bool {

                    }
                    WarehouseDetails { grid, current_position: potential_coords}
                } else {
                    WarehouseDetails { grid, current_position}
                }
            });
    0
}

fn main() {
    let input_grid: &Grid<char> = &read_grid_default("./resources/test_grid.csv").expect("Successful input parsing");
    let input_instructions: &Vec<Coords> = &read_csv::<String>("./resources/test.csv", b' ')
        .expect("Successful input parsing")
        .iter()
        .flat_map(|x| x.chars())
        .map(|x| match x {
            '^' => Coords::NORTH,
            'v' => Coords::SOUTH,
            '>' => Coords::EAST,
            '<' => Coords::WEST,
            _ => panic!("Unexpected case")
        })
        .collect();

    println!("Part 1: {:?}", input_instructions);
}



