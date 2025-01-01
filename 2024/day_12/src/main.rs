use common::grid::Grid;
use common::reader::read_grid_default;

trait AccumulatorDetails {}
impl AccumulatorDetails for AreaPerimeterDetails {}
struct AreaPerimeterDetails {}


fn create_visited_grid<T>(input: Grid<T>) -> Grid<bool> {
    Grid::<bool>{ inner: input.inner.iter().map(|x|x.iter().map(|_| false).collect()).collect() }
}

fn find_regions_value<F>(accumulator: F) -> i32
where F: FnMut() -> B{

}

fn main() {
    let input: Grid<char> = read_grid_default("./resources/test.csv").expect("Successful input read");
    let part_1 = input.clone();
    println!("Part 1: {:?}", part_1);
    let part_2 = input.clone();
    println!("Part 2: {:?}", part_2)
}
