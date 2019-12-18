
/// This is used to keep nine directions:
///
/// # Arguments
///
/// * `x` - The horizontal offset from the origin
/// * `y` - The vertical offset from the origin
/// * `cardinal`  - North, East, South, West.
/// * `active` - Used to specify four-connected or eight-connected neighborhoods.
///
/// # Example
///
/// ```
/// /// NorthEast
/// use compass_direction::CompassDirection;
/// let ne = CompassDirection {x : 1, y : -1, cardinal : false, active : true}; // NorthEast
/// ```
pub struct CompassDirection {
    pub x : isize,
    pub y : isize,
    pub cardinal : bool,
    pub active : bool,
}


pub struct Compass {
    compass : Vec<CompassDirection>
}

impl Compass {

    pub fn new(cardinals_only : bool) -> Compass {
        let compass : Vec<CompassDirection> = vec![
            CompassDirection {x : -1, y : -1, cardinal : false, active : !cardinals_only}, // NorthWest
            CompassDirection {x : 0, y : -1, cardinal : true, active : true}, // North
            CompassDirection {x : 1, y : -1, cardinal : false, active : !cardinals_only}, // NorthEast
            CompassDirection {x : -1, y : 0, cardinal : true, active : true}, // West
            CompassDirection {x : 0, y : 0, cardinal : false, active : true}, // Origin
            CompassDirection {x : 1, y : 0, cardinal : true, active : true}, // East
            CompassDirection {x : -1, y : 1, cardinal : false, active : !cardinals_only}, // SouthWest
            CompassDirection {x : 0, y : 1, cardinal : true, active : true}, // South
            CompassDirection {x : 1, y : 1, cardinal : false, active : !cardinals_only}, // SouthEast
        ];
        return Compass{compass : compass};
    }

    // pub fn get_cardinals(&self) -> Vec<CompassDirection> {
    //     let mut cardinals : Vec<CompassDirection> = Vec::<CompassDirection>::new();
    //     for direction in self.compass {
    //         if direction.cardinal {
    //             cardinals.push(direction);
    //         }
    //     }
    //     return cardinals;
    // }
    //
    // pub fn get_active(&self) -> Vec<CompassDirection>{
    //     let mut active_directions = Vec::<CompassDirection>::new();
    //     for direction in self.compass {
    //         if direction.active {
    //             active_directions.push(direction);
    //         }
    //     }
    //     return active_directions;
    // }
}
