
pub mod build {
    use crate::models::tetris::Tetris;
    use crate::models::tetris::RowCol;
    use rand::Rng;

    pub fn build_random_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let mut rng = rand::thread_rng();
        let rand_num = rng.gen_range(0..=4);
        match rand_num {
            0 => build_square_tetris(origin_row, origin_col),
            1 => build_tee_tetris(origin_row, origin_col),
            2 => build_zaa_tetris(origin_row, origin_col),
            3 => build_zee_tetris(origin_row, origin_col),
            4 => build_long_tetris(origin_row, origin_col),
            _ => build_zee_tetris(origin_row, origin_col),
        }
    }

    // origin: o->[ ], centre: c->[ ]
    //co[X], [X]
    //  [X], [X]
    fn build_square_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            RowCol {row: origin_row, col: origin_col},
            RowCol {row: origin_row, col: origin_col},
            RowCol {row: origin_row + 1, col: origin_col},
            RowCol {row: origin_row, col: origin_col + 1},
            RowCol {row: origin_row + 1, col: origin_col + 1},
            ];

        Tetris::new(poses, RowCol {row: 0, col: 0}, RowCol {row: origin_row, col: origin_col}, 0, 1)
    }

    // origin: o->[ ], centre: c->[ ]
    // >[ ], [X], [ ]
    //  [X],c[X], [X]
    fn build_tee_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            RowCol {row: origin_row, col: origin_col + 1}, 
            RowCol {row: origin_row + 1, col: origin_col}, 
            RowCol {row: origin_row + 1, col: origin_col + 1}, 
            RowCol {row: origin_row + 1, col: origin_col + 2},
            ];

        Tetris::new(poses, RowCol {row: 1, col: 1}, RowCol {row: origin_row, col: origin_col}, 0, 2)
    }

    // origin: o->[ ], centre: c->[ ]
    // o[ ], [X], [X]
    //  [X],c[X], [ ]
    fn build_zee_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            RowCol {row: origin_row, col: origin_col + 1}, 
            RowCol {row: origin_row, col: origin_col + 2},
            RowCol {row: origin_row + 1, col: origin_col}, 
            RowCol {row: origin_row + 1, col: origin_col + 1}, 
            ];

        Tetris::new(poses, RowCol {row: 1, col: 1}, RowCol {row: origin_row, col: origin_col}, 0, 5)
    }

    // origin: o->[ ], centre: c->[ ]
    // o[X],c[X], [ ]
    //  [ ], [X], [X]
    fn build_zaa_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            RowCol {row: origin_row, col: origin_col}, 
            RowCol {row: origin_row, col: origin_col + 1}, 
            RowCol {row: origin_row + 1, col: origin_col + 1}, 
            RowCol {row: origin_row + 1, col: origin_col + 2},
            ];

        Tetris::new(poses, RowCol {row: 0, col: 1}, RowCol {row: origin_row, col: origin_col}, 0, 4)
    }

    // origin: o->[ ], centre: c->[ ]
    // o[X],c[X], [X], [X]
    //  [ ], [ ], [ ], [ ]
    fn build_long_tetris(origin_row: usize, origin_col: usize) -> Tetris {
        let poses = vec![
            RowCol {row: origin_row, col: origin_col}, 
            RowCol {row: origin_row, col: origin_col + 1}, 
            RowCol {row: origin_row, col: origin_col + 2}, 
            RowCol {row: origin_row, col: origin_col + 3}, 
            ];

        Tetris::new(poses, RowCol {row: 0, col: 1}, RowCol {row: origin_row, col: origin_col}, 0, 6)
    }
}