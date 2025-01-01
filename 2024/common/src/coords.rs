
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coords {
    pub x: i32,
    pub y: i32
}


impl Coords {
    pub const NORTH: Coords = Coords {x: 0, y: -1};
    pub const EAST: Coords = Coords { x: 1, y: 0 };
    pub const SOUTH: Coords = Coords { x: 0, y: 1 };
    pub const WEST: Coords = Coords { x: -1, y: 0 };
    pub const NORTH_EAST: Coords = Coords { x: 1, y: -1 };
    pub const NORTH_WEST: Coords = Coords { x: -1, y: -1 };
    pub const SOUTH_EAST: Coords = Coords { x: 1, y: 1 };
    pub const SOUTH_WEST: Coords = Coords { x: -1, y: 1 };
    pub const CARDINALS: [Coords; 4] = [
        Coords::NORTH, Coords:: EAST, Coords:: SOUTH, Coords::WEST
    ];


    pub const DIAGONALS: [Coords; 4] = [
        Coords::NORTH_EAST, Coords::NORTH_WEST, Coords::SOUTH_EAST, Coords::SOUTH_WEST
    ];

    pub const DIRECTIONS: [Coords; 8] = [
        Coords::NORTH, Coords:: EAST, Coords:: SOUTH, Coords::WEST,
        Coords::NORTH_EAST, Coords::NORTH_WEST, Coords::SOUTH_EAST, Coords::SOUTH_WEST
    ];

    pub const RIGHT_ANGLES: [(Coords, Coords, Coords); 4] = [
        (Coords::NORTH, Coords::EAST, Coords::NORTH_EAST),
        (Coords::NORTH, Coords::WEST, Coords::NORTH_WEST),
        (Coords::SOUTH, Coords::EAST, Coords::SOUTH_EAST),
        (Coords::SOUTH, Coords::WEST, Coords::SOUTH_WEST)
    ];

    pub fn subtract_other(&self, other: Coords) -> Coords {
        Coords { x: self.x - other.x, y: self.y - other.y }
    }
}
