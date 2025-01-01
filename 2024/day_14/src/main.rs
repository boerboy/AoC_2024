use common::coords::Coords;
use common::reader::read_csv;
use regex::Regex;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct RobotDetails {
    position: Coords,
    velocity: Coords,
    robot_number: i32
}

fn parse_coords(re: &Regex, input: &String) -> Coords {
    re.captures_iter(input)
        .map(|capture| Coords {
            x: capture[1].parse().unwrap(),
            y: capture[2].parse().unwrap(),
        })
        .next()
        .unwrap()
}

fn parse_input() -> Vec<RobotDetails> {
    let re = &Regex::new(r".*(-?\d+),(-?\d+)").expect("Successful regex instantiation");
    read_csv::<(String, String)>("./resources/test.csv", b' ')
        .expect("Successful input read")
        .iter()
        .enumerate()
        .map(|(index, (position_str, velocity_str))| {
            let position = parse_coords(re, position_str);
            let velocity = parse_coords(re, velocity_str);
            RobotDetails { position, velocity, robot_number: index as i32 + 1}
        })
        .collect()
}

fn wrap_out_of_bounds(value: i64, max_non_zero: i32) -> i64 {
    if value > max_non_zero as i64 - 1i64 {
        value - max_non_zero as i64
    } else if value < 0i64 {
        max_non_zero as i64 + value
    } else {
        value
    }
}

fn walk_robots(input: &Vec<RobotDetails>, seconds: i32, width: i32, length: i32) -> Vec<RobotDetails> {
    let mut cloned = input.clone();
    (0..seconds).for_each(|_| {
        cloned.iter_mut().for_each(|x| {
            let new_coords = x.position.add(x.velocity);

            *x = RobotDetails {
                position: Coords {
                    x: wrap_out_of_bounds(new_coords.x, width),
                    y: wrap_out_of_bounds(new_coords.y, length)
                },
                velocity: x.velocity,
                robot_number: x.robot_number
            };
        })
    });
    cloned
}

fn calc_safety_factor(robots: Vec<RobotDetails>, width: i32, length: i32) -> i32 {
    let (middle_x, middle_y) = (width/2, length/2);
    robots.iter().fold((0, 0, 0, 0) |acc|)
}

fn main() {
    let input = &parse_input();
    let part_1 = walk_robots(input, 100, 11, 7);
    println!("Part 1: {:?}", part_1);
    let part_2 = 0;
    println!("Part 2: {:?}", part_2)
}
