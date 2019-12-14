extern crate petgraph;

use petgraph::unionfind::UnionFind;

use std::collections::HashMap;

use std::fmt;
use std::fmt::Write as FmtWrite;

pub struct Grid { // todo : rename to grid2d
    width : usize,
    height : usize,
    grid : Vec<usize>
}

impl Grid {

    pub fn new(width : usize, height : usize) -> Grid {
        // returns new empty grid

        let grid : Vec<usize> = vec![0; (width * height) as usize];

        return Grid {
                width : width,
                height : height,
                grid : grid
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

        // assumes universe is flattened based on width, height
        let mut uf = UnionFind::<usize>::new((self.width * self.height) as usize);

        for row_index in 0..self.height {
            for cell_index in 0..self.width {

                // println!("row_index:{}|cell_index:{}|total_index:{}", row_index, cell_index, (row_index * width + cell_index));
                let cell : usize = self.get_value(row_index, cell_index);

                if cell == 1 {
                    // foreground
                    // println!{"foreground"};
                    let mut labels : Vec<usize> = Vec::new();
                    // get neighboring labels
                    // todo : should be able to rewrite all of this to reuse offset = (x, y) code
                    if row_index != 0 { // row above
                        // north
                        let north_cell : usize = self.get_value(row_index - 1, cell_index);
                        // println!("north_cell:{}", north_cell);
                        if north_cell == 1 { // foreground
                            let label : usize = uf.find((row_index - 1) * self.width + cell_index);
                            // println!{"\tlabel:{}", label};
                            labels.push(label);
                        }
                    }
                    if cell_index != 0 { // left column
                        // west
                        let west_cell : usize = self.get_value(row_index, cell_index - 1);
                        if west_cell == 1 { // foreground
                            let label : usize = uf.find(row_index * self.width + (cell_index - 1));
                            labels.push(label);
                        }
                    }
                    if cell_index != self.width - 1 { // right column
                        let north_cell : usize = self.get_value(row_index, cell_index + 1);
                        if north_cell == 1 { // foreground
                            let label : usize = uf.find(row_index * self.width + (cell_index + 1));
                            labels.push(label);
                        }
                    }
                    if row_index != self.height - 1 { // row below
                        // south
                        let south_cell : usize = self.get_value(row_index + 1, cell_index);
                        if south_cell == 1 { // foreground
                            let label : usize = uf.find((row_index + 1) * self.width + cell_index);
                            labels.push(label);
                        }
                    }
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
            grid : self.get_connected_components(make_consecutive_labels)
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
