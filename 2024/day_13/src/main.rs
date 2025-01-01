use common::coords::Coords;
use regex::Regex;
use std::error::Error;
use rayon::prelude::*;

fn get_input(input: &str) -> Result<Vec<GameDetails>, Box<dyn Error>> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)",
    )?;
    Ok(input
        .to_string()
        .split("\n\n")
        .filter_map(|section| {
            re.captures(section).map(|capture| {
                let button_a = Coords {
                    x: capture[1].parse().unwrap(),
                    y: capture[2].parse().unwrap(),
                };
                let button_b = Coords {
                    x: capture[3].parse().unwrap(),
                    y: capture[4].parse().unwrap(),
                };
                let prize = Coords {
                    x: capture[5].parse().unwrap(),
                    y: capture[6].parse().unwrap(),
                };
                GameDetails {button_a, button_b, prize }
            })
        })
        .collect())
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct GameDetails { button_a: Coords, button_b: Coords, prize: Coords }

// Greatest common divisor
fn find_solution(
    button_a: Coords,
    button_b: Coords,
    prize: Coords
) -> (f64, f64){
    let divisor: f64=  (button_a.x * button_b.y - button_a.y * button_b.x) as f64;
    let a = (button_b.y * prize.x - button_b.x * prize.y) as f64 / divisor;
    let b = (-button_a.y * prize.x + button_a.x * prize.y) as f64 / divisor;
    (a, b)
}

fn find_prize(details: &GameDetails) -> i64 {
    let (solution_a, solution_b) = find_solution(details.button_a, details.button_b, details.prize);
    if solution_a.fract() == 0f64 && solution_b.fract() == 0f64 {
        3 * solution_a as i64 + solution_b as i64
    } else {
        0
    }
}

fn find_prize_increased(details: &GameDetails) -> i64 {
    let new_prize = details.prize.add_const(10000000000000);
    let new_details = GameDetails {button_a: details.button_a, button_b: details.button_b, prize: new_prize};
    find_prize(&new_details)
}

fn main() {
    let input: &Vec<GameDetails> = &get_input(include_str!("../resources/input.csv")).expect("Successful input read");
    let part_1: i64 = input.par_iter().map(|x|find_prize(x)).sum();
    println!("Part 1: {:?}", part_1);
    let part_2: i64 =input.par_iter().map(|x|find_prize_increased(x)).sum();
    println!("Part 2: {:?}", part_2)
}
