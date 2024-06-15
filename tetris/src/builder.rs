pub mod tetris {
    pub struct Tetris {
        pub poses: Vec<(usize, usize)>,
        pub color: i32,
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
            (origin_row + 1, origin_col + 1),
            ];

        Tetris {
            poses: poses,
            color: 4,
        }
    }
}