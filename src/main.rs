#![crate_name = "gol_rs"]

mod tca2d;
mod boundary;
mod compass_direction;
mod grid;
mod eca;

use boundary::BoundaryType;
use tca2d::TCA2D;
use grid::Grid;
use eca::ECA;

extern crate minifb;
use minifb::{Key, WindowOptions, Window, Scale};

use std::fmt;
use std::fmt::Write as FmtWrite;

use rand::prelude::*;

use std::thread;
use std::time::{Duration, Instant}; // timing

use std::env;

fn random_seed(width : usize) -> Vec::<usize> {

    let mut initial_configuration : Vec::<usize> = Vec::<usize>::new();

    for i in 0..width {
        if rand::random() {
            initial_configuration.push(1);
        } else {
            initial_configuration.push(0);
        }
    }
    return initial_configuration;
}

fn display_all_1d(width : usize, height : usize) {

    let generate_csv : bool = false;
    let generate_images : bool = true;
    let resize : bool = false;
    let connected_component_labelling : bool = true;

    let seed = random_seed(width);

    eca::generate_all_rules(width, height, seed, generate_csv, generate_images, resize, connected_component_labelling);

    // eca::generate_rule(129, width, height, seed, generate_csv, generate_images, resize, connected_component_labelling);
}

fn display_all_2d(width : usize, height : usize) {

    let depth : usize = 1;

    for code in 0..1000 {
        for moore in 0..2 {
            for outer_totalistic in 0..2 {


                let boundary_type : BoundaryType = BoundaryType::Null;
                let mut initial_configuration : Grid = Grid::new(width, height);
                // populate initial configuration; random seed
                for row_index in 0..height {
                    for column_index in 0..width {
                        if rand::random() {
                            initial_configuration.set_value(row_index, column_index, 1);
                        } else {
                            initial_configuration.set_value(row_index, column_index, 0);
                        }
                    }
                }

                let mut gol = TCA2D::new(code,
                    width,
                    height,
                    depth,
                    outer_totalistic != 0,
                    moore != 0,
                    boundary_type,
                    initial_configuration);
                    gol.display_infinite();
            }
        }
    }
}


fn main() {

    // let args : Vec<String> = env::args().collect();
    // println!("Args: {:?}", args);
    // let command = args[1].clone();

    let start = Instant::now();

    let mut code : usize = 224;
    // let width : usize = 200;
    // let height : usize = 130;
    // let width : usize = 1600; // good with x2
    let width : usize = 3600;
    let height : usize = 5400;
    // let height : usize = 10;
    let depth : usize = 1;
    let mut moore : bool = true;
    let mut outer_totalistic : bool = true;
    let boundary_type : BoundaryType = BoundaryType::Null;

    let mut initial_configuration : Grid = Grid::new(width, height);

    // populate initial configuration; random seed
    for row_index in 0..height {
        for column_index in 0..width {
            if rand::random() {
                initial_configuration.set_value(row_index, column_index, 1);
            } else {
                initial_configuration.set_value(row_index, column_index, 0);
            }
        }
    }

    let mut gol = TCA2D::new(code,
        width,
        height,
        depth,
        outer_totalistic,
        moore,
        boundary_type,
        initial_configuration);

    // gol.generate();
    // gol.generate_display();
    // gol.display_infinite();

    display_all_1d(width, height);

    // display_all_2d(800, 400);

    // println!("{}", gol);

    println!("runtime : {} ns", start.elapsed().as_nanos());
}
