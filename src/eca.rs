extern crate petgraph;
extern crate image;

use crate::grid::Grid;

use std::fmt;
use std::fmt::Write as FmtWrite;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

const RULES : [u8; 88] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 18, 19, 22, 23,
    24, 25, 26, 27, 28, 29, 30, 32, 33, 34, 35, 36, 37, 38, 40, 41, 42, 43, 44, 45, 46, 50, 51,
    54, 56, 57, 58, 60, 62, 72, 73, 74, 76, 77, 78, 90, 94, 104, 105, 106, 108, 110, 122, 126,
    128, 130, 132, 134, 136, 138, 140, 142, 146, 150, 152, 154, 156, 160, 162, 164, 168, 170,
    172, 178, 184, 200, 204, 232];

// todo : would be nice to have all the equivalence classes available too.
// todo : would be nice to have hash with universality type

pub struct ECA {
    // pub states : u8, // number of states
    // pub radius : u8, //
    // pub lattice : Vec<u32>,
    rule : u8,
    width : usize,
    height : usize,
    initial_configuration : Vec<usize>,
    universe : Grid
}

impl ECA {

    pub fn new(rule : u8, width : usize, height : usize, initial_configuration : Vec<usize>) -> ECA {

        // initialize universe with initial configuration
        let mut universe : Grid = Grid::new(width, height);

        for (column_index, cell) in initial_configuration.iter().enumerate() {
            let row_index = 0;
            universe.set_value(row_index, column_index, *cell);
        }

        return ECA {
            rule : rule,
            width : width,
            height : height,
            initial_configuration : initial_configuration.clone(),
            universe : universe
        };
    }

    pub fn get_value(&self, row_index : usize, column_index : usize) -> usize{
        return self.universe.get_value(row_index, column_index);
    }

    pub fn set_value(&mut self, row_index : usize, column_index : usize, value : usize) {
        self.universe.set_value(row_index, column_index, value);
    }

    fn increase_generation(&mut self, row_index : usize) {
        for cell_index in 0..(self.width) {
            // we look behind to the previous row to generate this row
            let radius : u8 = self.get_radius(row_index - 1, cell_index);
            let new_bit : usize = self.rule_lookup(radius);
            self.set_value(row_index, cell_index, new_bit);
        }
    }

    pub fn generate(&mut self) {
        for row_index in 1..self.height {
            self.increase_generation(row_index);
        }
    }

    fn get_radius(&self, row_index : usize, column_index : usize) -> u8 {

         let is_periodic : bool = false; // todo : eventually pass through

         let mut a : usize = self.get_value(
             row_index,
             (column_index +
                 if column_index == 0 {self.width} else {0}) - 1
                 % self.width);
         let b : usize = self.get_value(row_index, column_index);
         let mut c : usize = self.get_value(row_index, (column_index + 1) % self.width);

         if is_periodic == false {
             if (column_index +
                 if column_index == 0 {self.width} else {0} - 1) % self.width == 0 {
                 a = (a ^ a) & 1;
             }
             if (column_index + 1) % self.width == self.width - 1 {
                 c = (c ^ c) & 1;
             }
         }

         return (a << 2 | b << 1 | c) as u8; // the new rule, as a number
    }

    fn rule_lookup(&self, rule : u8) -> usize{
        return ((self.rule & (1 << rule)) >> rule) as usize; // todo : maybe rename rule to shift
    }

    pub fn get_flattened_universe(&self) -> Vec<u8> {
        let mut universe : Vec<u8> = Vec::new();

        for row_index in 0..self.height {
            for column_index in 0..self.width {
                let cell : usize = self.get_value(row_index, column_index);

                let a : u8 = (cell & 255) as u8;
                let b : u8 = (cell & (255 << 8)) as u8;
                let c : u8 = (cell & (255 << 16)) as u8;
                let d : u8 = (cell & (255 << 24)) as u8;

                if cell == 0 {
                    universe.push(0xff);
                    universe.push(0xff);
                    universe.push(0xff);
                    universe.push(0xff);
                } else {
                    universe.push(a);
                    universe.push(b);
                    universe.push(c);
                    universe.push(0xff);
                }
            }
        }

        return universe;
    }

    pub fn generate_connected_components(&mut self) {
        self.universe.label_connected_components();
    }

    pub fn save_to_csv(&self) -> Result<(), Box<dyn std::error::Error>> {
        return self.universe.save_to_csv();
    }

    pub fn reset(&mut self) {
        // initialize universe with initial configuration
        let mut universe : Grid = Grid::new(self.width, self.height);

        for (column_index, cell) in self.initial_configuration.iter().enumerate() {
            let row_index = 0;
            universe.set_value(row_index, column_index, *cell);
        }

        self.universe = universe;
    }

}

impl fmt::Display for ECA {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        return write!(f, "Rule : {} | universe : \n{} ", self.rule, self.universe);
    }
}

pub fn generate_rule(rule : u8, width : usize, height : usize, seed : Vec<usize>, generate_csv : bool, generate_images : bool, resize : bool, ccl : bool) {

    let filename = format!("./rule{}length{}.png", &rule.to_string(), &width.to_string());

    let mut automata = ECA::new(rule as u8, width, height, seed);
    automata.generate();

    if ccl {
        automata.generate_connected_components();
        //get_connected_components(width, height, &automata.universe);
    }

    if generate_images {
        let buffer = automata.get_flattened_universe();
        image::save_buffer(&filename, buffer.as_slice(), width as u32, height as u32, image::RGBA(8)).unwrap();

        if resize { // extremely slow
            // resize
            let mut im = image::open(&filename).unwrap();
            // let mut fout = File::open(filename);
            let fout = &mut File::create(&Path::new(&filename)).unwrap();
            // Write the contents of this image to the Writer in PNG format.
            // im.thumbnail(800, 600).write_to(fout, image::PNG).unwrap();
            im.resize(1063, 1375, image::FilterType::Nearest).write_to(fout, image::PNG).unwrap();
        }
    }

    if generate_csv {
        automata.save_to_csv().unwrap();
    }

}

pub fn generate_all_rules(width : usize, height : usize, seed : Vec<usize>, generate_csv : bool, generate_images : bool, resize : bool, ccl : bool) {

    // inequivalent rules
    // for rule in RULES.iter() {
    //     generate_rule(*rule, width, height, generate_csv, generate_images, resize, ccl);
    // }

    // todo : should be able to loop and just rewrite the same automata object; then I don't
    //          have to keep reallocating the universe data on the heap

    for rule in 0..256 {
        generate_rule(rule as u8, width, height, seed.clone(), generate_csv, generate_images, resize, ccl);
    }
}



// todo : write function that takes a random seed and generates all nonequiv rules for it.

// todo : write function that takes a 255 random seeds and runs all nonequiv rules for them.

// todo : add proper boundary code

// todo : add insert code, after increasing generation
