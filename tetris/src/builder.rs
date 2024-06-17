pub mod tetris {
    use std::cmp::{max, min};

    pub struct RowCol {
        pub col: usize,
        pub row: usize,
    }

    pub struct Tetris {
        pub poses: Vec<RowCol>,
        pub centre: RowCol,
        pub shift: RowCol,
        pub color: usize,
    }

    impl Tetris {
        pub fn get_poses_shifted(&self) -> Vec<RowCol> {
            let mut shifted_poses = vec![];
            for row_col in &self.poses {
                shifted_poses.push(RowCol {row: row_col.row + self.shift.row, col: row_col.col + self.shift.col});
            }
            return shifted_poses
        }

        pub fn move_tetris(&mut self, grid: &Vec<Vec<usize>>, direction: &(i32, i32)) -> bool {
            if *direction == (0, 0) {
                return true
            }

            let mut new_poses = vec![];
            let num_cols = grid.get(0).unwrap().len() as i32;
            let num_rows = grid.len() as i32;
            for row_col in &self.poses {
                let row = &row_col.row;
                let col = &row_col.col;
                let new_row = *row as i32 - direction.0;
                let new_col = *col as i32 - direction.1;

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

                let new_pos = RowCol {
                    row: new_row as usize,
                    col: new_col as usize,
                };
                new_poses.push(new_pos);
            }

            self.poses = new_poses;
            return true
        }

        pub fn drop_tetris(&mut self, grid: &Vec<Vec<usize>>) {
            while self.move_tetris(grid, &(-1, 0)) {}
        }

        // reset to 0, 0
        pub fn reset_tetris(&mut self) {
            let mut new_poses = vec![];
            let mut smallest_row = usize::MAX;
            let mut smallest_col = usize::MAX;
            for row_col in &self.poses {
                smallest_row = min(row_col.row, smallest_row);
                smallest_col = min(row_col.col, smallest_col);
            }
            for row_col in &self.poses {
                let new_pos = RowCol {
                    row: row_col.row - smallest_row,
                    col: row_col.col - smallest_col,
                };
                new_poses.push(new_pos);
            }
            self.poses = new_poses;
        }

        pub fn spin_tetris(&mut self, spin: i32) {
            let centre = self.get_centre();
            let mut new_poses: Vec<(i32, i32)> = vec![];
            let mut smallest_row = i32::MAX;
            let mut largest_row = 0;
            let mut smallest_col = i32::MAX;
            let mut largest_col = 0;
            for row_col in &self.poses {
                let norm_pos = (row_col.row as i32 - centre.0 as i32, row_col.col as i32 - centre.1 as i32);
                let norm_spun = (norm_pos.1 * -1 * spin, norm_pos.0 * spin);
                let spun = (norm_spun.0 + centre.0 as i32, norm_spun.1 + centre.1 as i32);

                smallest_row = min(spun.0, smallest_row);
                largest_row = max(spun.0, largest_row);
                smallest_col = min(spun.1, smallest_col);
                largest_col = max(spun.1, largest_col);

                new_poses.push(spun);
            }
            
            let mut shift = (0, 0);
            if smallest_row < 0 {
                shift.0 = 0 - smallest_row;
            } else if largest_row > 19 {
                shift.0 = 9 - largest_row;
            }
            if smallest_col < 0 {
                shift.1 = 0 - smallest_col;
            } else if largest_col > 9 {
                shift.1 = 19 - largest_col;
            }

            let mut new_shifted_poses = vec![];
            for poses in &new_poses {
                let shifted_pos = RowCol {
                    row: (poses.0 + shift.0) as usize,
                    col: (poses.1 + shift.1) as usize,
                };
                new_shifted_poses.push(shifted_pos);
            }

            self.poses = new_shifted_poses;
        }

        fn get_centre(&self) -> (usize, usize) {
            let mut smallest_row = usize::MAX;
            let mut largest_row = 0;
            let mut smallest_col = usize::MAX;
            let mut largest_col = 0;
            for row_col in &self.poses {
                smallest_row = min(row_col.row, smallest_row);
                largest_row = max(row_col.row, largest_row);
                smallest_col = min(row_col.col, smallest_col);
                largest_col = max(row_col.col, largest_col);
            }
            let centre_row = (smallest_row + largest_row) / 2;
            let centre_col = (smallest_col + largest_col) / 2;
            (centre_row, centre_col)
        }
    }
}

pub mod build {
    use crate::builder::tetris::Tetris;
    use rand::{prelude, Rng};

    use super::tetris::RowCol;

    pub fn build_random_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let mut rng = rand::thread_rng();
        let rand_num: f64 = rng.gen();
        let rand_num_i = (rand_num * 4.99) as i32;
        match rand_num_i {
            0 => build_square_tetris(origin_row, origin_col),
            1 => build_tee_tetris(origin_row, origin_col),
            2 => build_zaa_tetris(origin_row, origin_col),
            3 => build_zee_tetris(origin_row, origin_col),
            4 => build_long_tetris(origin_row, origin_col),
            _ => build_zee_tetris(origin_row, origin_col),
        }
    }

    // origin is top left corner.
    // >[X], [X]
    //  [X], [X]
    fn build_square_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            RowCol {row: origin_row, col: origin_col},
            RowCol {row: origin_row, col: origin_col},
            RowCol {row: origin_row + 1, col: origin_col},
            RowCol {row: origin_row, col: origin_col + 1},
            RowCol {row: origin_row + 1, col: origin_col + 1},
            ];

        Tetris {
            poses: poses,
            centre: RowCol {row: origin_row, col: origin_col},
            shift: RowCol {row: 0, col: 0},
            color: 1,
        }
    }

    // origin is top left corner, which is empty.
    // >[ ], [X], [ ]
    //  [X], [X], [X]
    fn build_tee_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            RowCol {row: origin_row, col: origin_col + 1}, 
            RowCol {row: origin_row + 1, col: origin_col}, 
            RowCol {row: origin_row + 1, col: origin_col + 1}, 
            RowCol {row: origin_row + 1, col: origin_col + 2},
            ];

        Tetris {
            poses: poses,
            centre: RowCol {row: origin_row, col: origin_col},
            shift: RowCol {row: 0, col: 0},
            color: 2,
        }
    }

    // origin is top left corner, which is empty.
    // >[ ], [X], [X]
    //  [X], [X], [ ]
    fn build_zee_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            RowCol {row: origin_row, col: origin_col + 1}, 
            RowCol {row: origin_row, col: origin_col + 2},
            RowCol {row: origin_row + 1, col: origin_col}, 
            RowCol {row: origin_row + 1, col: origin_col + 1}, 
            ];

        Tetris {
            poses: poses,
            centre: RowCol {row: origin_row, col: origin_col},
            shift: RowCol {row: 0, col: 0},
            color: 5,
        }
    }

    // origin is top left corner, which is empty.
    // >[X], [X], [ ]
    //  [ ], [X], [X]
    fn build_zaa_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            RowCol {row: origin_row, col: origin_col}, 
            RowCol {row: origin_row, col: origin_col + 1}, 
            RowCol {row: origin_row + 1, col: origin_col + 1}, 
            RowCol {row: origin_row + 1, col: origin_col + 2},
            ];

        Tetris {
            poses: poses,
            centre: RowCol {row: origin_row, col: origin_col},
            shift: RowCol {row: 0, col: 0},
            color: 4,
        }
    }

    // origin is top left corner, which is empty.
    // >[X], [X], [X], [X]
    //  [ ], [ ], [ ], [ ]
    fn build_long_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            RowCol {row: origin_row, col: origin_col}, 
            RowCol {row: origin_row, col: origin_col + 1}, 
            RowCol {row: origin_row, col: origin_col + 2}, 
            RowCol {row: origin_row, col: origin_col + 3}, 
            ];

        Tetris {
            poses: poses,
            centre: RowCol {row: origin_row, col: origin_col},
            shift: RowCol {row: 0, col: 0},
            color: 6,
        }
    }
}