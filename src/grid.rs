extern crate petgraph;

use crate::compass_direction::{CompassDirection, Compass};

use petgraph::unionfind::UnionFind;

use std::collections::HashMap;

use std::fmt;
use std::fmt::Write as FmtWrite;

pub enum Connectivity {
    FourConnected,
    SixConnected, // for hexagonal grid
    EightConnected
}

// todo : move the boundary object in here?

pub struct Grid { // todo : rename to grid2d
    width : usize,
    height : usize,
    grid : Vec<usize>,
    connectivity : Connectivity,
    // compass : Compass // todo : pass a compass into the grid
    // todo : boundary type?
}

impl Grid {

    pub fn new(width : usize, height : usize) -> Grid {
        // returns new empty grid

        // todo : add to new constructor
        let connectivity : Connectivity = Connectivity::FourConnected;

        let grid : Vec<usize> = vec![0; (width * height) as usize];

        return Grid {
                width : width,
                height : height,
                grid : grid,
                connectivity : connectivity
        };
    }

    pub fn get_grid(&self) -> &Vec<usize> {
        return &self.grid;
    }

    pub fn get_value(&self, row_index : usize, column_index : usize) -> usize {
        return self.grid[row_index * self.width + column_index];
    }

    pub fn set_value(&mut self, row_index : usize, column_index : usize, value : usize) {
        self.grid[row_index * self.width + column_index] = value;
    }

    pub fn get_connected_components(&self, make_consecutive_labels : bool) -> Vec<usize> {
        // todo : rewrite this to calculate connected components based on the boundary

        // assumes universe is flattened based on width, height
        let mut uf = UnionFind::<usize>::new((self.width * self.height) as usize);

        let eight_connected : bool = match self.connectivity {
            Connectivity::EightConnected => true,
            _ => false
        };

        for row_index in 0..self.height {
            for cell_index in 0..self.width {

                // println!("row_index:{}|cell_index:{}|total_index:{}", row_index, cell_index, (row_index * width + cell_index));
                let cell : usize = self.get_value(row_index, cell_index);

                if cell == 1 {
                    // foreground
                    // println!{"foreground"};

                    // todo : move all the rest to a different method; get_labels

                    let mut labels : Vec<usize> = Vec::new();
                    // get neighboring labels
                    // todo : should be able to rewrite all of this to reuse offset = (x, y) code

                    // north
                    if row_index != 0 { // row above

                        let north_cell : usize = self.get_value(row_index - 1, cell_index);
                        // println!("north_cell:{}", north_cell);
                        if north_cell == 1 { // foreground
                            let label : usize = uf.find((row_index - 1) * self.width + cell_index);
                            // println!{"\tlabel:{}", label};
                            labels.push(label);
                        }
                    }

                    // west
                    if cell_index != 0 { // left column

                        let west_cell : usize = self.get_value(row_index, cell_index - 1);
                        if west_cell == 1 { // foreground
                            let label : usize = uf.find(row_index * self.width + (cell_index - 1));
                            labels.push(label);
                        }
                    }

                    if eight_connected {

                        // north west
                        if row_index != 0 && cell_index != 0 {
                            let north_west_cell : usize = self.get_value(row_index - 1, cell_index - 1);

                            if north_west_cell == 1 { // foreground
                                let label : usize = uf.find((row_index - 1) * self.width + (cell_index - 1));
                                labels.push(label);
                            }
                        }

                        // north east
                        if row_index != 0 && cell_index < self.width - 1 {
                            let north_east_cell : usize = self.get_value(row_index - 1, cell_index + 1);

                            if north_east_cell == 1 { // foreground
                                let label : usize = uf.find((row_index - 1) * self.width + (cell_index + 1));
                                labels.push(label);
                            }
                        }

                    }

                    // todo : check : not necessary?
                    // if cell_index != self.width - 1 { // right column
                    //     // east
                    //     let north_cell : usize = self.get_value(row_index, cell_index + 1);
                    //     if north_cell == 1 { // foreground
                    //         let label : usize = uf.find(row_index * self.width + (cell_index + 1));
                    //         labels.push(label);
                    //     }
                    // }
                    // if row_index != self.height - 1 { // row below
                    //     // south
                    //     let south_cell : usize = self.get_value(row_index + 1, cell_index);
                    //     if south_cell == 1 { // foreground
                    //         let label : usize = uf.find((row_index + 1) * self.width + cell_index);
                    //         labels.push(label);
                    //     }
                    // }
                    for label in &labels {
                        uf.union(row_index * self.width + cell_index, *label);
                        // println!{"\tlabel:{}", label};
                    }
                } else {
                    // background
                    // println!{"background"};
                    uf.union(0, row_index * self.width + cell_index);
                }
            }
        }
        // let mut universe_str = String::new();
        let mut universe_labelled : Vec<usize> = uf.into_labeling();
        //
        // for (index, cell) in universe_labelled.iter().enumerate() {
        //     universe_str.push_str(&cell.to_string());
        //
        //     if index % width as usize == (width - 1) as usize {
        //         universe_str.push('\n');
        //     } else {
        //         universe_str.push(',');
        //     }
        // }
        // println!("{}", universe_str);

        // use consecutive integers.

        if make_consecutive_labels {
            let mut consecutive_labels = HashMap::<usize, usize>::new();

            let mut label_counter = 1;

            for cell in &universe_labelled {
                if *cell != 0 {
                    if !consecutive_labels.contains_key(cell) {
                        consecutive_labels.insert(*cell, label_counter);
                        label_counter = label_counter + 1;
                    }
                } else {
                    consecutive_labels.insert(*cell, 0);
                }
            }

            // println!("number of components : {}", consecutive_labels.len() - 1);

            for (index, cell) in universe_labelled.iter_mut().enumerate() {
                *cell = consecutive_labels[cell];
            }
        }

        return universe_labelled;
    }

    pub fn get_connected_components_grid(&self) -> Grid{
        let make_consecutive_labels : bool = true;

        return Grid {
            width : self.width,
            height : self.height,
            grid : self.get_connected_components(make_consecutive_labels),
            connectivity : match self.connectivity {
                Connectivity::FourConnected => Connectivity::FourConnected,
                Connectivity::SixConnected => Connectivity::SixConnected,
                Connectivity::EightConnected => Connectivity::EightConnected,
                _ =>Connectivity::FourConnected
            }
            // compass : self.compass
        };
    }

    pub fn label_connected_components(&mut self) {
        let make_consecutive_labels : bool = true;

        self.grid = self.get_connected_components(make_consecutive_labels);
    }

    pub fn get_number_of_components(&self) -> usize {
        // assuming you've run relabelling.
        let mut max : usize = 0;
        for cell in &self.grid {
            if *cell > max {
                max = *cell;
            }
        }
        return max
    }

    pub fn get_filter_l1(&mut self) {
        let max_label = self.width + self.height;

        for label in 1..(max_label + 1) {

            // generate coordinates instead!

            for row_index in 0..self.height {
                for column_index in 0..self.width {

                    // check if cell == label
                    let cell : usize = self.get_value(row_index, column_index);
                    if cell == label {
                        // look nesw and give the label value plus 1.
                        // for direction in self.compass.get_cardinals() {

                        }

                }
            }
        }
            // update grid
    }


    pub fn filter_l2(&self) {

    }

    pub fn filter_linf(&self) {

    }

    pub fn save_to_dipha(&self) {

    }

    pub fn save_to_csv(&self) -> Result<(), Box<dyn std::error::Error>>{
        // This assumes you've run generate.

        // let mut csv_filename = String::new();
        // writeln!(&mut csv_filename, "./rule{}|size{}|seed{}.csv", self.rule, self.width, 1)?;

        let csv_filename = format!("./width{}|height{}.csv", self.width, self.height);

        let grid_as_string : String = self.get_grid_str();

        // println!("{}", universe_as_string);
        std::fs::write(csv_filename, grid_as_string).unwrap();

        return Ok(());
    }

    pub fn get_grid_str(&self) -> String {
        let mut grid_as_string : String = String::new();

        for (index, cell) in self.grid.iter().enumerate() {
            if *cell != 0 {
                grid_as_string.push_str(&cell.to_string());
                // println!("cell : {}", 1);
            } else {
                grid_as_string.push('0');
                // println!("cell : {}", 0);
            }

            if index % self.width as usize == (self.width - 1) as usize {
                grid_as_string.push('\n');
            } else {
                grid_as_string.push(',');
            }
        }

        return grid_as_string;
    }

    // todo : add rotate grid method
}

impl fmt::Display for Grid {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        return write!(f, "{}", self.get_grid_str());
    }
}

// todo write function to return grid with single 1 in the middle.

// todo write function to return grid of a specific game of life object, based on the lexicon.
