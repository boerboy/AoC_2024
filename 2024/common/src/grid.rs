use crate::coords::Coords;

pub struct Grid<T> {
    pub inner: Vec<Vec<T>>
}

impl<T> Grid<T>
where
    T: PartialEq,
    T: Copy {
    pub fn find(&self, search_char: T) -> Vec<Coords> {
        self.inner.iter().enumerate().flat_map(|(y, value)| {
            value.iter().enumerate().flat_map(move |(x, value)| {
                if *value == search_char {
                    Some(Coords{x: x as i32, y: y as i32})
                } else {
                    None
                }
            })
        }).collect()
    }
}