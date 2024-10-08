use crate::models::tetris::Tetris;

pub struct Grid {
    pub grid_vec: Vec<Vec<usize>>,
}

impl Grid {
    pub fn apply_tetris(&mut self, tetris: &Tetris) -> i32 {
        for row_col in &tetris.get_poses() {
            let row = row_col.row;
            let col = row_col.col;
            *self.grid_vec
                .get_mut(row).unwrap()
                .get_mut(col).unwrap() = tetris.color;
        }

        let mut lines_cleared = 0;
        let cp_grid = self.grid_vec.clone();
        for (index, row) in cp_grid.iter().enumerate() {
            if !row.contains(&0) {
                lines_cleared += 1;
                self.grid_vec.remove(index);
                self.grid_vec.insert(0, vec![0; 10]);
            }
        }

        return lines_cleared
    }
}
