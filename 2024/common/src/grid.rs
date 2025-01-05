use std::fmt::{Debug, Display};
use crate::coords::Coords;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Grid<T> {
    pub inner: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: PartialEq,
    T: Copy,
    T: Debug,
    T: Display
{
    pub fn find(&self, search_value: T) -> Vec<Coords> {
        self.inner
            .iter()
            .enumerate()
            .flat_map(|(y, value)| {
                value.iter().enumerate().flat_map(move |(x, value)| {
                    if *value == search_value {
                        Some(Coords {
                            x: x as i64,
                            y: y as i64,
                        })
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn find_predicate_preserve<F>(&self, search_fn: F) -> Vec<(&T, Coords)>
    where
        F: Copy,
        F: Fn(&T) -> bool {
        self.inner
            .iter()
            .enumerate()
            .flat_map(|(y, value)| {
                value.iter().enumerate().flat_map(move |(x, value)| {
                    if search_fn(value) {
                        Some((value, Coords {
                            x: x as i64,
                            y: y as i64,
                        }))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn find_one(&self, search_char: T) -> Option<Coords> {
        self.find(search_char).get(0).map(|x|*x)
    }

    pub fn value_at_mut(&mut self, coords: Coords) -> Option<&mut T> {
        self.inner
            .get_mut(coords.y as usize)
            .map(|x| x.get_mut(coords.x as usize))
            .flatten()
    }

    pub fn value_at(&self, coords: Coords) -> Option<&T> {
        self.inner
            .get(coords.y as usize)
            .map(|x| x.get(coords.x as usize))
            .flatten()
    }

    pub fn look_for_by_coords_delta(
        &self,
        current_coords: Coords,
        search: T,
        deltas: &[Coords],
    ) -> Vec<Coords> {
        deltas
            .iter()
            .flat_map(|delta| {
                let next_coords = current_coords.subtract(*delta);
                let value = &self.value_at(next_coords);
                match value {
                    Some(x) if **x == search => Some(next_coords),
                    _ => None,
                }
            })
            .collect()
    }

    pub fn update(&mut self, coords: Coords, value: T) -> bool {
        if self.value_at(coords).is_some() {
            self.inner[coords.y as usize][coords.x as usize] = value;
            true
        } else {
            false
        }
    }

    pub fn fetch_by_deltas(&self, current_coords: Coords, deltas: &[Coords]) -> Vec<(Coords, &T)> {
        deltas
            .iter()
            .flat_map(|delta| {
                let next_coords = current_coords.subtract(*delta);
                self.value_at(next_coords).map(|x|(next_coords,x))
            })
            .collect()
    }

    pub fn fetch_at(&self, coords: Coords) -> Option<&T> {
        self.inner.get(coords.y as usize).map(|row|row.get(coords.x as usize)).flatten()
    }



    pub fn fetch_by_delta(&self, current_coords: Coords, delta: &Coords) -> Option<&T> {
        let next_coords = current_coords.subtract(*delta);
        self.value_at(next_coords)
    }

    pub fn create_default(bounds: Coords, default: T) -> Grid<T> {
        Grid {
            inner: (0..bounds.y).map(|_| {
                (0..bounds.x).map(|_|default).collect()
            }).collect()
        }
    }

    pub fn pretty_print(&self) -> () {
        self.inner.iter().for_each(|row| {
            row.iter().for_each(|value| print!("{}", value));
            println!()
        })
    }
}
