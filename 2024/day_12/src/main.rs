use common::coords::Coords;
use common::grid::Grid;
use common::reader::read_grid_default;

struct AccumulatorDetails {
    coords: Coords,
    crop: char,
}

fn accumulate_area_and_perimeter(input: &Grid<char>, details: AccumulatorDetails) -> (i32, i32) {
    let surrounding_count = input.look_for_by_coords_delta(details.coords, details.crop, &Coords::CARDINALS).iter().count();
    (4 - surrounding_count as i32, 1)
}

fn accumulate_sides_and_area(input: &Grid<char>, details: AccumulatorDetails) -> (i32, i32) {
    Coords::RIGHT_ANGLES.iter().fold((1, 0), |(area, sides), (longitudinal, latitudinal, diagonal)| {
        let x = input.fetch_by_delta(details.coords, longitudinal).map(|x| *x);
        let y =  input.fetch_by_delta(details.coords, latitudinal).map(|x| *x);
        let z =  input.fetch_by_delta(details.coords, diagonal).map(|x| *x);
        if x != Some(details.crop) && y != Some(details.crop) {
            (area, sides + 1)
        }  else if x == Some(details.crop) && y == Some(details.crop) && z != Some(details.crop){
            (area, sides + 1)
        } else {
            (area, sides)
        }
    })
}

fn create_visited_grid<T>(input: &Grid<T>) -> Grid<bool> {
    Grid::<bool> {
        inner: input
            .inner
            .iter()
            .map(|x| x.iter().map(|_| false).collect())
            .collect(),
    }
}

fn traverse_region<F>(
    input: &Grid<char>,
    visited: &mut Grid<bool>,
    details: AccumulatorDetails,
    accumulator: &F
) -> i32
where
    F: Fn(&Grid<char>, AccumulatorDetails) -> (i32, i32),
{
    fn inner<F>(i: &Grid<char>, v: &mut Grid<bool>, d: AccumulatorDetails, f: &F, acc: (i32, i32)) -> (i32, i32)
    where
        F: Fn(&Grid<char>, AccumulatorDetails) -> (i32, i32),
    {
        if *v.value_at(d.coords).unwrap_or(&true) {
            acc
        } else {
            v.update(d.coords, true);
            
            let (surrounding_factor, surrounding_other_factor) = i
                .look_for_by_coords_delta(d.coords, d.crop, &Coords::CARDINALS)
                .iter()
                .fold((0, 0), |(factor_0, other_factor_0), coords| {
                    let details = AccumulatorDetails { coords: *coords, crop: d.crop};
                    let (inner_factor, inner_other_factor) = inner(i, v, details, f, acc);
                    (factor_0 + inner_factor, other_factor_0 + inner_other_factor)
                });
            let (factor, other_factor) = acc;
            let (factor_0, other_factor_0) = f(i, d);
            (factor + factor_0 + surrounding_factor, other_factor + other_factor_0 + surrounding_other_factor)
        }
    }
    let (factor, other_factor) = inner(input, visited, details, accumulator, (0, 0));
    factor * other_factor
}

fn find_regions_value<F>(input: &Grid<char>, accumulator: &F) -> i32
where
    F: Fn(&Grid<char>, AccumulatorDetails) -> (i32, i32),
{
    let visited = &mut create_visited_grid(input);
    input
        .inner
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_index, crop)| AccumulatorDetails {
                    coords: Coords {
                        x: col_index as i32,
                        y: row_index as i32,
                    },
                    crop: *crop,
                })
        })
        .fold(0, |acc, details| {
            if *visited.value_at(details.coords).unwrap_or(&true) {
                acc
            } else {
                acc + traverse_region(input, visited, details, accumulator)
            }
        })
}

fn main() {
    let input: Grid<char> =
        read_grid_default("./resources/input.csv").expect("Successful input read");
    let part_1 = find_regions_value(&input, &accumulate_area_and_perimeter);
    println!("Part 1: {:?}", part_1);
    let part_2 = find_regions_value(&input, &accumulate_sides_and_area);
    println!("Part 2: {:?}", part_2)
}
