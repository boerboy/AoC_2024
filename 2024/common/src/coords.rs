
#[derive(Copy, Clone)]
pub struct Coords {
    pub x: i32,
    pub y: i32
}


impl Coords {
    pub const CARDINALS: [Coords; 4] = [
        // North
        Coords {x: 0, y: -1},
        // East
        Coords { x: 1, y: 0 },
        // South
        Coords { x: 0, y: 1 },
        // West
        Coords { x: -1, y: 0 }
    ];


    pub const DIAGONALS: [Coords; 4] = [
        // North East
        Coords { x: 1, y: -1 },
        // North West
        Coords { x: -1, y: -1 },
        // South East
        Coords { x: 1, y: 1 },
        // South West
        Coords { x: -1, y: 1 },
    ];

    pub fn directions() -> Vec<Coords> { [Coords::CARDINALS, Coords::DIAGONALS].concat() }
}
