use std::ops::Deref;
use common::coords::Coords;
use common::grid::Grid;
use common::reader::{read_csv, read_grid_default};

struct WarehouseDetails<'a> {
    boxes: &'a mut Vec<(char, Coords)>,
    walls: &'a Vec<Coords>,
    current_position: Coords,
}

fn can_move(
    boxes: &Vec<(char, Coords)>,
    walls: &Vec<Coords>,
    potential_coords: Coords,
    instruction: &Coords,
) -> bool {
    let box_ = boxes.iter().find(|(_, x)| *x == potential_coords);
    let wall = walls.iter().find(|x| **x == potential_coords);
    if wall.is_some() {
        false
    } else if box_.is_some() {
        match box_ {
            Some(('[', _)) if *instruction == Coords::NORTH || *instruction == Coords::SOUTH =>
                can_move(boxes,
                         walls,
                         potential_coords.add(*instruction),
                         instruction) &&
                    can_move(boxes,
                             walls,
                             potential_coords.add(*instruction).add(Coords::EAST),
                             instruction)
            ,
            Some((']', _)) if *instruction == Coords::NORTH || *instruction == Coords::SOUTH =>
                can_move(boxes,
                         walls,
                         potential_coords.add(*instruction),
                         instruction) &&
                    can_move(boxes,
                             walls,
                             potential_coords.add(*instruction).add(Coords::WEST),
                             instruction),
            _ => can_move(boxes,
                          walls,
                          potential_coords.add(*instruction),
                          instruction)
        }
    } else {
        true
    }
}

fn find_with_index(input: &Vec<(char, Coords)>, search: Coords) -> Option<(usize, &(char, Coords))> {
    input
        .iter()
        .enumerate()
        .find(|(_, (_, x))| *x == search)
}

fn hitting_wall(walls: &Vec<Coords>, search: Coords) -> Option<bool> {
    if walls.iter().find(|x| **x == search).is_none() {
        Some(true)
    } else {
        None
    }
}

fn move_vertically_expanded(details: (usize, &(char, Coords)),
                            walls: &Vec<Coords>,
                            potential_coords: Coords,
                            boxes: &mut Vec<(char, Coords)>,
                            cloned: &Vec<(char, Coords)>,
                            side: Coords,
                            instruction: &Coords) -> Option<()> {
    let new_coords = potential_coords.add(*instruction);
    let next_box = cloned.iter().find(|(_, x)| *x == new_coords);
    let (index, (char, _)) = details;
    let _ = hitting_wall(walls, potential_coords.add(side));
    let (other_index, (other_char, other_coords)) = find_with_index(cloned, potential_coords.add(side))?;
    let new_other_coords = other_coords.add(*instruction);
    *boxes.get_mut(other_index).expect("Box exists") = (*other_char, new_other_coords);
    *boxes.get_mut(index).expect("Box exists") = (*char, new_coords);
    if next_box.is_some() {
        move_boxes(boxes, walls, new_coords, instruction);
        move_boxes(boxes, walls, new_other_coords, instruction);
    }
    Some(())
}

fn move_boxes(
    boxes: &mut Vec<(char, Coords)>,
    walls: &Vec<Coords>,
    potential_coords: Coords,
    instruction: &Coords,
) -> Option<()> {
    let cloned = &boxes.clone();
    let _ = hitting_wall(walls, potential_coords);
    let details @ (index, (char, _)) = find_with_index(cloned, potential_coords)?;
    match (char, instruction) {
        ('[', _) if *instruction == Coords::NORTH || *instruction == Coords::SOUTH =>
            move_vertically_expanded(details, walls, potential_coords, boxes, cloned, Coords::EAST, instruction),
        (']', _) if *instruction == Coords::NORTH || *instruction == Coords::SOUTH =>
            move_vertically_expanded(details, walls, potential_coords, boxes, cloned, Coords::WEST, instruction),
        _ => {
            let new_coords = potential_coords.add(*instruction);
            let next_box = cloned.iter().find(|(_, x)| *x == new_coords);
            *boxes.get_mut(index).expect("Box exists") = (*char, new_coords);
            if next_box.is_some() {
                move_boxes(boxes, walls, new_coords, instruction);
            };
            Some(())
        }
    }
}

fn attempt_walk<'a>(
    boxes: &'a mut Vec<(char, Coords)>,
    walls: &'a Vec<Coords>,
    current_position: Coords,
    instruction: &'a Coords,
) -> WarehouseDetails<'a> {
    let potential_coords = current_position.add(*instruction);
    if can_move(boxes, walls, potential_coords, instruction) {
        move_boxes(boxes, walls, potential_coords, instruction);
        WarehouseDetails {
            boxes,
            walls,
            current_position: potential_coords,
        }
    } else {
        WarehouseDetails {
            boxes,
            walls,
            current_position,
        }
    }
}

fn walk(grid: &Grid<char>, instructions: &Vec<Coords>) -> i64 {
    let start = grid.find_one('@').expect("Start exists");
    let boxes: Vec<(char, Coords)> = grid.find_predicate_preserve(|x| match x {
        'O' => true,
        '[' => true,
        ']' => true,
        _ => false,
    }).iter().map(|(x, y)| (**x, *y)).collect();
    let walls = grid.find('#');
    instructions
        .iter()
        .fold(
            WarehouseDetails {
                boxes: &mut boxes.clone(),
                walls: &walls.clone(),
                current_position: start,
            },
            |WarehouseDetails {
                 boxes,
                 walls,
                 current_position,
             },
             instruction| {
                attempt_walk(boxes, walls, current_position, instruction)
            },
        )
        .boxes
        .iter()
        .map(|(_, x)| x.x + x.y * 100)
        .sum()
}

fn expand_and_walk(grid: &Grid<char>, instructions: &Vec<Coords>) -> i64 {
    let expanded_grid: &Grid<char> = &Grid {
        inner: {
            grid.inner.iter().fold(Vec::new(), |mut acc, row| {
                let expanded_row: Vec<char> = row
                    .iter()
                    .flat_map(|x| match *x {
                        '.' => vec!['.', '.'],
                        '#' => vec!['#', '#'],
                        'O' => vec!['[', ']'],
                        '@' => vec!['@', '.'],
                        _ => panic!("Unhandled case"),
                    })
                    .collect();
                acc.push(expanded_row);
                acc
            })
        },
    };
    walk(expanded_grid, instructions)
}

fn main() {
    let input_grid: &Grid<char> =
        &read_grid_default("./resources/test_grid.csv").expect("Successful input parsing");
    let input_instructions: &Vec<Coords> = &read_csv::<String>("./resources/test.csv", b' ')
        .expect("Successful input parsing")
        .iter()
        .flat_map(|x| x.chars())
        .map(|x| match x {
            '^' => Coords::NORTH,
            'v' => Coords::SOUTH,
            '>' => Coords::EAST,
            '<' => Coords::WEST,
            _ => panic!("Unexpected case"),
        })
        .collect();
    // let part_1 = walk(input_grid, input_instructions);
    // println!("Part 1: {:?}", part_1);
    let part_2 = expand_and_walk(input_grid, input_instructions);
    println!("Part 2: {:?}", part_2);
}
