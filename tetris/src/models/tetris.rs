use std::cmp::{max, min};

pub struct RowCol {
    pub col: usize,
    pub row: usize,
}

pub struct Tetris {
    tiles: Vec<RowCol>,
    centre: RowCol,
    spin: i32, // 0 - 3, 0 being upright, and 3 being 270 degrees spun.
    shift: RowCol,
    pub color: usize,
}

impl Tetris {
    pub fn new(tiles: Vec<RowCol>, centre: RowCol, shift: RowCol, spin: i32, color: usize) -> Self {
        Tetris {tiles, centre, shift, spin, color}
    }

    pub fn get_tiles(&self) -> &Vec<RowCol> {
        &self.tiles
    }

    pub fn get_poses(&self) -> Vec<RowCol> {
        let mut shifted_poses = vec![];
        for row_col in &self.tiles {
            let norm_row = row_col.row as i32 - self.centre.row as i32;
            let norm_col = row_col.col as i32 - self.centre.col as i32;
            let mut norm_spun = (norm_row, norm_col);
            for _i in 0..self.spin {
                norm_spun = (-norm_spun.1, norm_spun.0);
            }
            let abs_row = (norm_spun.0 + self.centre.row as i32 + self.shift.row as i32) as usize;
            let abs_col = (norm_spun.1 + self.centre.col as i32 + self.shift.col as i32) as usize;
            let spun = RowCol {
                row: abs_row,
                col: abs_col,
            };

            shifted_poses.push(spun);
        }
        return shifted_poses
    }

    pub fn move_tetris(&mut self, grid: &Vec<Vec<usize>>, direction: &(i32, i32)) -> bool {
        // check if we hit floor
        if *direction == (0, 0) {
            return true
        }
        let num_cols = grid.get(0).unwrap().len() as i32;
        let num_rows = grid.len() as i32;
        for row_col in &self.get_poses() {
            let row = &row_col.row;
            let col = &row_col.col;
            let new_row = *row as i32 + direction.0;
            let new_col = *col as i32 + direction.1;

            if new_col < 0 || new_col >= num_cols {
                return true
            }
            if new_row < 0 || new_row >= num_rows {
                return false
            }
            if *grid
                .get(new_row as usize).unwrap()
                .get(*col).unwrap()
                != 0 as usize {
                return false
            }
            if *grid
                .get(new_row as usize).unwrap()
                .get(new_col as usize).unwrap()
                != 0 as usize {
                return true
            }
        }

        let new_shift = RowCol {
            row: (self.shift.row as i32 + direction.0) as usize,
            col: (self.shift.col as i32 + direction.1) as usize,
        };
        self.shift = new_shift;
        return true
    }

    pub fn drop_tetris(&mut self, grid: &Vec<Vec<usize>>) {
        while self.move_tetris(grid, &(1, 0)) {}
    }

    // reset to 0, 0
    pub fn reset_tetris(&mut self) {
        self.shift = RowCol {row: 0, col: 0};
        self.spin = 0;
    }

    pub fn spin_tetris(&mut self, spin: i32) {
        let new_spin = (self.spin + spin + 4) % 4;
        let mut row_shift = 0;
        let mut col_shift = 0;
        // check we are not colliding with anything
        for row_col in &self.tiles {
            let norm_row = row_col.row as i32 - self.centre.row as i32;
            let norm_col = row_col.col as i32 - self.centre.col as i32;
            let mut norm_spun = (norm_row, norm_col);
            for _i in 0..new_spin {
                norm_spun = (-norm_spun.1, norm_spun.0);
            }
            let abs_row = norm_spun.0 + self.centre.row as i32 + self.shift.row as i32;
            let abs_col = norm_spun.1 + self.centre.col as i32 + self.shift.col as i32;
            if abs_row < 0 {
                row_shift = max(row_shift, -abs_row);
            } else if abs_row > 19 {
                row_shift = min(row_shift, 19 - abs_row);
            }
            if abs_col < 0 {
                col_shift = max(col_shift, -abs_col);
            } else if abs_col > 9 {
                col_shift = min(col_shift, 9 - abs_col);
            }
        }

        let new_shift_row = (self.shift.row as i32 + row_shift) as usize;
        let new_shift_col = (self.shift.col as i32 + col_shift) as usize;
        self.shift = RowCol {
            row: new_shift_row,
            col: new_shift_col,
        };
        self.spin = new_spin;
    }
}