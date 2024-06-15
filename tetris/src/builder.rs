pub mod tetris {
    pub struct Tetris {
        pub poses: Vec<(usize, usize)>,
        pub color: i32,
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
    }
}

pub mod build {
    use crate::builder::tetris::Tetris;
    // origin is top left corner.
    // >[X], [X]
    //  [X], [X]
    pub fn build_square_tetris(origin_row: usize, origin_col: usize) -> Tetris {
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
    // >[O], [X], [ ]
    //  [X], [X], [X]
    pub fn build_tee_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            (origin_row, origin_col + 1), 
            (origin_row + 1, origin_col), 
            (origin_row + 1, origin_col + 1), 
            (origin_row + 1, origin_col + 2),
            ];

        Tetris {
            poses: poses,
            color: 4,
        }
    }
}