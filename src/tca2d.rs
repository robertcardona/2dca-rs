use crate::boundary::{Boundary, BoundaryType};
use crate::compass_direction::CompassDirection;
use crate::grid::Grid;

extern crate minifb;
use minifb::{Key, WindowOptions, Window, Scale};

use std::fmt;
use std::fmt::Write as FmtWrite;

use std::thread;
use std::time::{Duration}; // timing


// todo : moore and outer_totalistic don't need to be part of the class;
//      make part of the methods that use them.
pub struct TCA2D {
    rule : usize,
    width : usize,
    height : usize,
    depth : usize,
    moore : bool, // only needed to generate compass
    outer_totalistic : bool, // no : is totalistic.
    compass : Vec<CompassDirection>, // "constant"
    boundary : Boundary, // type to use for calculations
    universe : Vec<Grid>
}

impl TCA2D { // totalistic cellular automata : 2 dimensional

    pub fn new(rule : usize,
        width : usize,
        height : usize,
        depth : usize,
        outer_totalistic : bool,
        moore : bool,
        boundary_type : BoundaryType,
        initial_configuration : Grid) -> TCA2D {

        // initialize universe with initial configuration
        let mut universe : Vec<Grid> = Vec::<Grid>::new();

        // allocate memory

        universe.push(initial_configuration); // pass by reference

        for i in 1..depth {
            let page : Grid = Grid::new(width, height);

            universe.push(page);
        }

        // object to use for calculations!
        let boundary = Boundary::new(width, height, boundary_type);

        // generate the valid compass based on moore or von_neumann
        // order is important
        let compass : Vec<CompassDirection> = vec![
            CompassDirection {x : -1, y : -1, cardinal : false, active : moore}, // NorthWest
            CompassDirection {x : 0, y : -1, cardinal : true, active : true}, // North
            CompassDirection {x : 1, y : -1, cardinal : false, active : moore}, // NorthEast
            CompassDirection {x : -1, y : 0, cardinal : true, active : true}, // West
            CompassDirection {x : 0, y : 0, cardinal : false, active : true}, // Origin
            CompassDirection {x : 1, y : 0, cardinal : true, active : true}, // East
            CompassDirection {x : -1, y : 1, cardinal : false, active : moore}, // SouthWest
            CompassDirection {x : 0, y : 1, cardinal : true, active : true}, // South
            CompassDirection {x : 1, y : 1, cardinal : false, active : moore}, // SouthEast
        ];

        // todo : I can implement extended von_neumann here (in future)

        return TCA2D {
            rule : rule,
            width : width,
            height : height,
            depth : depth,
            moore : moore, // delete
            outer_totalistic : outer_totalistic,
            compass : compass,
            boundary : boundary,
            universe : universe
        };
    }

    pub fn get_value(&self, page_index : usize, row_index : usize, column_index : usize) -> usize {
        return self.universe[page_index].get_value(row_index, column_index);
    }

    pub fn set_value(&mut self, page_index : usize, row_index : usize, column_index : usize, value : usize) {
        self.universe[page_index].set_value(row_index, column_index, value);
    }

    //
    fn get_radius_at_index(&self, page_index : usize, row_index : usize, column_index : usize) -> Vec<usize> {
        // this returns the *actual* values at the valid grid coordinates.
        // this will be called for every pixel at every page, so it needs to be efficient.


        // based on boundary_type
        return match self.boundary.boundary_type {
            BoundaryType::Null => self.get_null_radius_at_index(page_index, row_index, column_index),
            _ => Vec::<usize>::new()
        };

    }

    fn get_null_radius_at_index(&self,
        page_index : usize,
        row_index : usize,
        column_index : usize) -> Vec<usize> {
        // should return a vector with nine elements.

        let mut radius : Vec<usize> = vec![0; 9];

        // note : direction knows moore or von_neumann type!
        for (index, direction) in self.compass.iter().enumerate() {
            // get valid indices
            // if negative, value is zero
            // if nonnegative, access universe

            let mut direction_row_index : isize = -1;
            let mut direction_column_index : isize = -1;

            if direction.active {
                direction_row_index =
                    self.boundary.get_new_row_index(row_index, &direction);
                direction_column_index =
                    self.boundary.get_new_column_index(column_index, &direction);
            } else {
                // leave row_index, column_index = -1
            }

            // println!("dri:{}, dci:{}", direction_row_index, direction_column_index);

            // todo : maybe rewrite to >= 0?
            if direction_row_index != -1 && direction_column_index != -1 {
                // new calculated indices are valid
                // get the value in universe at the given page
                radius[index] = self.get_value(
                    page_index - 1,
                    direction_row_index as usize,
                    direction_column_index as usize);
            } else {
                // calculated indices are not valid, or cell not active
                radius[index] = 0;
            }
        }

        return radius;
    }

    pub fn get_next_page(&self, page_index : usize) -> Grid {

        // assumes page_index > 0
        // does not increase the page or set the value; you must do that yourself.

        let mut next_page : Grid = Grid::new(self.width, self.height);

        // here is where I need to know totalistic vs outer_totalistic!
        for row_index in 0..self.height {
            for column_index in 0..self.width {
                let mut cell : usize = 0;

                // get radius
                let radius : Vec<usize> =
                    self.get_radius_at_index(page_index, row_index, column_index);

                if self.outer_totalistic { // outer_totalistic
                    let a : u8 = radius[4] as u8;
                    let n : usize = radius.iter().sum();

                    cell = TCA2D::rule_lookup_outer_totalistic(a, n as u8 - a, self.rule);
                } else { // totalistic
                    // totalistic : n = sum of all nine cells
                    let n : usize = radius.iter().sum();
                    cell = TCA2D::rule_lookup_totalistic(n as u8, self.rule);
                }

                // set value
                next_page.set_value(row_index, column_index, cell);
            }
        }

        return next_page;
    }

    pub fn increase_generation(&mut self, page_index : usize) {
        let next_generation : Grid = self.get_next_page(page_index);

        for row_index in 0..self.height {
            for column_index in 0..self.width {
                self.set_value(page_index,
                    row_index,
                    column_index,
                    next_generation.get_value(row_index, column_index));
            }
        }
    }

    // This is used to insert a subgrid at a given page
    pub fn insert_at(&mut self,
        page_index : usize,
        row_index : usize,
        column_index : usize,
        width : usize,
        height : usize,
        subgrid : Vec<usize>) {
        // inserts `subgrid` at (page_index, row_index, column_index) of size (width, height)

        // assuming subgrid is size width * height

        // check if the subgrid fits at the given location
       if width + column_index <= self.width - 1 && height + row_index <= self.height {

           // insertion code
           for sub_row_index in 0..height {
               for sub_column_index in 0..width {

                   // self.universe[page_index][(height + row_index + sub_row_index) * self.width +
                   //      (width + column_index + sub_column_index)] =
                   //      subgrid[sub_row_index * width + sub_column_index];

                    // todo : check if this is works as desired :
                    self.set_value(page_index,
                        row_index + sub_row_index,
                        column_index + sub_column_index,
                        subgrid[sub_row_index * width + sub_column_index]);
               }
           }
       }
       // maybe return boolean : worked
    }

    pub fn insert_page(&mut self,
        page_index : usize,
        row_index : usize,
        column_index : usize,
        width : usize,
        height : usize,
        subgrid : Vec<usize>) {
        // ideally just call insert_at(self.width, self.height, )
        self.insert_at(page_index, row_index, column_index, width, height, subgrid);

    }

    // todo : rename to generate_finite?
    pub fn generate(&mut self) {
        for page_index in 1..self.depth {
            self.increase_generation(page_index);
        }
    }

    pub fn generate_display(&mut self) {
        for page_index in 1..self.depth {
            self.increase_generation(page_index);

            // todo : add code to display the page

            let mut buffer: Vec<u32> = vec![0; self.width * self.height];

            // copy page; cast as u32 for display object
            for row_index in 0..self.height {
                for column_index in 0..self.width {
                    buffer[row_index * self.width + column_index] =
                        if self.get_value(page_index, row_index, column_index) == 1 {
                            0xffffff
                        } else {
                            0x00
                        };
                }
            }

            let mut window = Window::new("Test - ESC to exit",
                                         self.width,
                                         self.height,
                                         WindowOptions {
                                            scale: Scale::X16,
                                            ..WindowOptions::default()
                                         }).unwrap_or_else(|e| {
                panic!("{}", e);
            });

            while window.is_open() && !window.is_key_down(Key::Escape) {

                // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
                window.update_with_buffer_size(&buffer, self.width, self.height).unwrap();
            }
        }
    }

    pub fn display_infinite(&mut self) {
        // assumes depth == 1

        let mut buffer: Vec<u32> = vec![0; self.width * self.height];

        let mut title = format!("code:{}|width:{}|height:{}|generation:{}",
            &self.rule.to_string(),
            &self.width.to_string(),
            &self.height.to_string(),
            0);

        let mut window = Window::new("Test - ESC to exit",
                                     self.width,
                                     self.height,
                                     WindowOptions {
                                        scale: Scale::X2,
                                        ..WindowOptions::default()
                                     }).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // todo : add code to generate a gif of what is being shown to user

        let mut generation : usize = 0;

        while window.is_open() && !window.is_key_down(Key::Escape) {

            let connected_components : Grid  = self.universe[0].get_connected_components_grid();
            // println!("{}", connected_components);

            // println!("#components:{}, generation:{}", connected_components.get_number_of_components(), generation);

            // update buffer
            for row_index in 0..self.height {
                for column_index in 0..self.width {
                    let cell : u32 = connected_components.get_value(row_index, column_index) as u32;

                    buffer[row_index * self.width + column_index] =
                        if cell != 0 {
                            cell * 1000
                        } else {
                            0x00
                        };
                }
            }

            // update universe
            let mut next_generation : Grid = self.get_next_page(1);
            self.universe[0] = next_generation;

            // update generation
            generation = generation + 1;
            title = format!("code:{}|moore:{}|totalistic:{}|width:{}|height:{}|generation:{}|#components:{}",
                &self.rule.to_string(),
                &self.moore,
                &self.outer_totalistic,
                &self.width.to_string(),
                &self.height.to_string(),
                generation.to_string(),
                connected_components.get_number_of_components());

            window.set_title(&title);
            window.update_with_buffer_size(&buffer, self.width, self.height).unwrap();

            // while(!window.is_key_down(Key::W)) {}
            let sleep_time = Duration::from_millis(100);
            // thread::sleep(sleep_time);   c
        }


    }

    /// Returns the component ~f(a, n) of an outer totalistic code.
    ///
    /// # Arguments
    ///
    /// * `a` : 0 or 1
    /// * `n` : 0, 1, ..., 8
    ///
    /// e.g., (outer) code = 224 -> f(1,2), f(0,3), f(1,3) = 1.
    fn rule_lookup_outer_totalistic(a : u8, n : u8, code : usize) -> usize {

        let shift = 2 * n + a;
        let mask = 1 << shift;

        // println!("code = {:#020b}", code);
        // println!("mask = {:#020b}", mask);

        return (mask & code) >> shift;
    }

    // Returns the component f(n) of a totalistic code.
    ///
    /// # Arguments
    ///
    /// * `n` : 0, 1, ..., 9
    ///
    /// e.g., code =
    fn rule_lookup_totalistic(n : u8, code : usize) -> usize {
        return (code & (1 << n)) >> n;
    }

    fn save_page_as_png(&self, page_index : usize) {

    }

    fn save_universe_as_png(&self) {

    }

    fn save_page_as_csv(&self, page_index : usize) {

    }

    fn save_universe_as_csv(&self) {

    }

    fn save_universe_as_gif(&self) {

    }

    /// Returns the page at a given index as a string
    fn get_page(&self, page_index : usize) -> String {
        let mut page_str = String::new();

        for (index, cell) in (self.universe[page_index].get_grid()).iter().enumerate() {
            page_str.push_str(&cell.to_string());
            if index % self.width as usize == (self.width - 1) as usize {
                page_str.push('\n');
            }
        }

        return page_str;
    }

    pub fn reset(&mut self) {
        // wipe everything except first page
        // alternatively, have a reset method that puts in a new initial_configuration
        //      save from having to reallocate memory for the entire universe
    }

}

impl fmt::Display for TCA2D {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.

        let mut universe_str : String = String::new();
        for page_index in 0..self.universe.len() {
            universe_str.push_str(&format!("page : {}\n", page_index));
            universe_str.push_str(&self.get_page(page_index));
            universe_str.push('\n');
        }

        return write!(f, "{}", universe_str);
    }
}

// there are two ways of generating the connected components : 2d or 3d.
// can look at connected components at the page level, or look at the universe as a
// 3d object.

// Returns a pair of integers that represent offsets from current cell in some
//
// # Arguments
//
// * offsets_name : Offsets
//
// e.g., Offsets::North = [-1, 0] = [row_offset, column_offset]
//

// fn get_offset(offset_name : Offsets) -> [isize; 2] {
//
//     return match offset_name {
//         Offsets::NorthWest => [-1, 1],
//         Offsets::North => [-1, 0],
//         Offsets::NorthEast => [-1, 1],
//         Offsets::East => [0, 1],
//         Offsets::SouthEast => [1, 1],
//         Offsets::South => [1, 0],
//         Offsets::SouthWest => [1, -1],
//         Offsets::West => [0, -1],
//         _ => [0,0]
//     }
// }
