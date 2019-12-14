
use crate::compass_direction::CompassDirection;

pub enum BoundaryType {
    Null,
    Cylinder,
    Moebius,
    Torus,
    Klein,
}

pub struct Boundary { // maybe rename to grid boundary
    width : usize,
    height : usize,
    pub boundary_type : BoundaryType
}

impl Boundary {
    // I'm wondering if this is too much overhead; for each bit, it will do all of these
    // checks, and it will create this boundary object :/

    pub fn new(width : usize, height : usize, boundary_type : BoundaryType) -> Boundary {
        return Boundary {
            width : width,
            height : height,
            boundary_type : boundary_type
        };
    }

    pub fn get_new_row_index(&self,
        row_index : usize,
        offset : &CompassDirection) -> isize {
        // returns the new coordinates, shifted from the given cell, following the

        // rules of the given BoundaryType
        let mut new_row_index : isize;

        new_row_index = match self.boundary_type {
            BoundaryType::Null => self.get_null_row_index(row_index, offset),
            _ => -1
        };

        return new_row_index;
    }

    pub fn get_new_column_index(&self,
        column_index : usize,
        offset : &CompassDirection) -> isize {
        // returns new column index based on the given directional shift.

        let mut new_column_index : isize;

        new_column_index = match self.boundary_type {
            BoundaryType::Null => self.get_null_column_index(column_index, &offset),
            _ => -1
        };

        return new_column_index;
    }

    fn get_null_row_index(&self,
        row_index : usize,
        offset : &CompassDirection) -> isize { // should cast as usize
        //

        let mut new_row_index : isize = -1;

        if (row_index == 0 && offset.y < 0) || (row_index == self.height - 1 && offset.y > 0) {
            // row == 0 and I'm trying to go to row == -1;
            // null rule : use zero.
        } else {
            new_row_index = (row_index as isize) + offset.y;
        }

        // if row_index == self.height - 1 && offset.y > 0 {
        //     // row == last row and trying to go one more;
        //     // null rule : use zero.
        // } else {
        //     new_row_index = (row_index as isize) + 1;
        // }

        return new_row_index;
    }


    fn get_null_column_index(&self,
        column_index : usize,
        offset : &CompassDirection) -> isize {
        // todo : add comments

        // println!("column_index:{}, offset.x:{}", column_index, offset.x);

        let mut new_column_index : isize = -1;

        if (column_index == 0 && offset.x < 0) ||
            (column_index == self.width - 1 && offset.x > 0)  {
            // column == 0 and trying to go to column == -1;
            // null rule : use zero
        } else {
            new_column_index = (column_index as isize) + offset.x;
        }

        // if column_index == self.width - 1 && offset.x > 0 {
        //     // column == last column and trying to go one more;
        //     // null rule : use zero.
        // } else {
        //     new_column_index = (column_index as isize) + offset.x;
        // }

        return new_column_index;
    }

}
