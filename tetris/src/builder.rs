pub mod tetris {
    use std::collections::HashMap;

    pub struct Tetris {
        pub poses: Vec<(usize, usize)>,
        pub color: usize,
    }

    impl Tetris {
        pub fn move_tetris(&mut self, grid: &Vec<Vec<usize>>, direction: &(i32, i32)) -> bool {
            if *direction == (0, 0) {
                return true
            }

            let mut new_poses = vec![];
            let num_cols = grid.get(0).unwrap().len() as i32;
            let num_rows = grid.len() as i32;
            for (row, col) in &self.poses {
                let new_col = *col as i32 - direction.1;
                let new_row = *row as i32 - direction.0;

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

                new_poses.push((new_row as usize, new_col as usize));
            }

            self.poses = new_poses;
            return true
        }

        pub fn drop_tetris(&mut self, grid: &Vec<Vec<usize>>) {
            while self.move_tetris(grid, &(-1, 0)) {}
        }
    }
}

pub mod build {
    use crate::builder::tetris::Tetris;
    use rand::{prelude, Rng};

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
            (origin_row, origin_col), 
            (origin_row + 1, origin_col), 
            (origin_row, origin_col + 1), 
            (origin_row + 1, origin_col + 1),
            ];

        Tetris {
            poses: poses,
            color: 1,
        }
    }

    // origin is top left corner, which is empty.
    // >[ ], [X], [ ]
    //  [X], [X], [X]
    fn build_tee_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            (origin_row, origin_col + 1), 
            (origin_row + 1, origin_col), 
            (origin_row + 1, origin_col + 1), 
            (origin_row + 1, origin_col + 2),
            ];

        Tetris {
            poses: poses,
            color: 2,
        }
    }

    // origin is top left corner, which is empty.
    // >[ ], [X], [X]
    //  [X], [X], [ ]
    fn build_zee_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            (origin_row, origin_col + 1), 
            (origin_row, origin_col + 2),
            (origin_row + 1, origin_col), 
            (origin_row + 1, origin_col + 1), 
            ];

        Tetris {
            poses: poses,
            color: 5,
        }
    }

    // origin is top left corner, which is empty.
    // >[X], [X], [ ]
    //  [ ], [X], [X]
    fn build_zaa_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            (origin_row, origin_col), 
            (origin_row, origin_col + 1), 
            (origin_row + 1, origin_col + 1), 
            (origin_row + 1, origin_col + 2),
            ];

        Tetris {
            poses: poses,
            color: 4,
        }
    }

    // origin is top left corner, which is empty.
    // >[X], [X], [X], [X]
    //  [ ], [ ], [ ], [ ]
    fn build_long_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            (origin_row, origin_col), 
            (origin_row, origin_col + 1), 
            (origin_row, origin_col + 2), 
            (origin_row, origin_col + 3), 
            ];

        Tetris {
            poses: poses,
            color: 6,
        }
    }
}