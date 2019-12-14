extern crate petgraph;

use petgraph::unionfind::UnionFind;

use std::collections::HashMap;


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

    pub fn get_grid(self) -> Vec<usize> {
        return self.grid;
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

            println!("number of components : {}", consecutive_labels.len() - 1);

            for (index, cell) in universe_labelled.iter_mut().enumerate() {
                *cell = consecutive_labels[cell];
            }
        }

        return universe_labelled;
    }

    pub fn label_connected_components(&mut self) {
        let make_consecutive_labels : bool = true;
        
        self.grid = self.get_connected_components(make_consecutive_labels);
    }
}
