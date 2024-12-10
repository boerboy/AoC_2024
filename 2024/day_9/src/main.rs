use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use itertools::Itertools;

fn read_csv() -> Result<String, Box<dyn Error>> {
    let file = File::open("./resources/input.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_reader(file);

    let result = rdr.records()
        .flat_map(|x| x)
        .map(|x| {
            String::from(x.iter().next().unwrap_or(""))
        })
        .next().unwrap_or(String::new());

    Ok(result)
}

fn build_disk(input: String) -> Vec<i32> {
    input.chars()
        .collect_vec()
        .chunks(2)
        .enumerate()
        .flat_map(|(index, chunk)| {
            let mut inner_vec = Vec::<i32>::new();
            let chunk_vec = chunk.to_vec();
            let data = chunk_vec.get(0usize).map(|x| x.to_string().parse().expect("i32")).unwrap_or(0);
            let spaces = chunk_vec.get(1usize).map(|x| x.to_string().parse().expect("i32")).unwrap_or(0);

            for _ in 0..data {
                inner_vec.push(index as i32)
            }
            for _ in 0..spaces {
                inner_vec.push(-1)
            }
            inner_vec
        })
        .collect_vec()
}

fn compact_disk(disk: Vec<i32>) -> Vec<i32> {
    let mut gaps = disk.iter().enumerate().filter(|(_, x)| **x == -1).map(|(index, _)| index).collect_vec();

    disk.iter().enumerate().rev().fold(disk.clone(), |mut acc, (index, x)| {
        let next_gap = gaps.get(0).unwrap_or(&usize::MAX);
        if *x != -1 && index > *next_gap {
            acc[*next_gap] = *x;
            acc[index] = -1;
            gaps.remove(0);
            gaps.push(index);
            acc
        } else {
            acc
        }
    })
}

fn build_chunks(disk: Vec<(usize, &i32)> ) -> Vec<(usize, (usize, &i32))> {
    let windows = disk.windows(2);
    let mut gaps: HashMap<usize, (usize, &i32)> = HashMap::new();
    let max = (usize::MAX, &i32::MAX);
    let (mut current_key, current_value) = windows.clone().next().unwrap_or(&[max, max])[0];
    let mut current_size = 1usize;
    gaps.insert(current_key, (current_size, current_value));
    for window in windows {
        let (previous_index, previous_value) = window[0];
        let (current_index, current_value) = window[1];
        if previous_index == current_index - 1 && previous_value == current_value {
            current_size += 1;
            gaps.insert(current_key, (current_size, current_value));
        } else {
            current_size = 1usize;
            current_key = current_index;
            gaps.insert(current_key, (current_size, current_value));
        }
    }
    gaps
        .iter()
        .map(|(x, y)| (*x, *y))
        .sorted_by(|(x, _), (y, _)| x.cmp(y))
        .collect_vec()
}

fn compact_disk_blocked_ordered(disk: Vec<i32>) -> Vec<i32> {
    let empties: Vec<(usize, &i32)> = disk
        .iter()
        .enumerate()
        .filter(|(_, x)| **x == -1)
        .collect_vec();

    let occupied: Vec<(usize, &i32)> = disk
        .iter()
        .enumerate()
        .filter(|(_, x)| **x != -1)
        .collect_vec();
    let mut gaps: Vec<(usize, (usize, &i32))> = build_chunks(empties);
    let chunks: Vec<(usize, (usize, &i32))> = build_chunks(occupied);
    chunks
        .iter()
        .sorted_by(|(_, (_, x)), (_, (_, y))| y.cmp(x))
        .fold(disk.clone(), |mut acc, (index, (size, value))| {
            let (gap_index, (next_gap_index, (gap_size, _))) = gaps
                .iter()
                .enumerate()
                .find(|(_, (_, (x, _)))| x >= size)
                .unwrap_or((usize::MAX, &(usize::MAX, (usize::MAX, &i32::MIN))));
            if index > next_gap_index && size <= gap_size {
                for i in *next_gap_index..next_gap_index + size {
                    acc[i] = **value;
                }
                for i in *index..index + size {
                    acc[i] = -1;
                }
                gaps[gap_index] = (next_gap_index+size,(gap_size - size, &-1));
                acc
            } else {
                acc
            }
        })
}

fn part_1(input: String) -> i64 {
    let disk = build_disk(input);
    let compacted_disk = compact_disk(disk);
    compacted_disk
        .iter()
        .enumerate()
        .filter(|(_, x)| **x != -1)
        .fold(0i64, |acc, (index, value)| acc + index as i64 * *value as i64)
}

fn part_2(input: String) -> i64 {
    let disk = build_disk(input);
    let compacted_disk = compact_disk_blocked_ordered(disk);
    compacted_disk
        .iter()
        .enumerate()
        .filter(|(_, x)| **x != -1)
        .fold(0i64, |acc, (index, value)| acc + index as i64 * *value as i64)
}

fn main() {
    let input = read_csv().expect("Successful input read");
    let part_1 = part_1(input.clone());
    println!("Part 1: {:?}", part_1);
    let part_2 = part_2(input.clone());
    println!("Part 2: {:?}", part_2)
}