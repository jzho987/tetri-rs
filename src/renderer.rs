pub mod render {
    extern crate colored;
    use colored::Colorize;

    extern crate futures_timer;

    extern crate crossterm;
    use crossterm::cursor;
    use crossterm::style::{Print, Stylize};
    use crossterm::terminal::{enable_raw_mode, Clear, ClearType};

    use std::io::{stdout, Stdout};

    use crate::models::grid::Grid;
    use crate::models::tetris::Tetris;

    // characters
    const CORNER_TOP_LEFT_CHAR: char = '╔';
    const CORNER_TOP_RIGHT_CHAR: char = '╗';
    const CORNER_BOT_LEFT_CHAR: char = '╚';
    const CORNER_BOT_RIGHT_CHAR: char = '╝';
    const BORDER_VERT_CHAR: char = '║';
    const BORDER_HORI_CHAR: char = '═';
    const ROW_START: i32 = 0;
    const COL_START: i32 = 0;
    const BACKGROUND_TOTAL_ROWS: i32 = (PADDING * 3) + MAIN_GRID_COLS + SECND_GRID_COLS;
    const BACKGROUND_TOTAL_COLS: i32 = (PADDING * 2) + MAIN_GRID_ROWS;

    // screen positions
    const MAIN_GRID_COLS: i32 = 10;
    const MAIN_GRID_ROWS: i32 = 20;
    const SECND_GRID_COLS: i32 = 5;
    const SECND_GRID_ROWS: i32 = 4;
    const SCORE_ROWS:i32 = 2;
    const PADDING: i32 = 1;

    pub struct Renderer {
        stdout: Stdout,
    }

    impl Renderer {
        pub fn new() -> Self {
            Renderer { 
                stdout: stdout(),
            }
        }

        pub fn render_background(&self) {
            let mut stdout = &self.stdout;
            // prep render
            enable_raw_mode().unwrap();
            execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

            // render background
            {
                for i in ROW_START..=BACKGROUND_TOTAL_ROWS {
                    for j in COL_START..=BACKGROUND_TOTAL_COLS {
                        let (cell, width) = get_cell(&3);
                        let row_pos = (width * i) as u16;
                        execute!(stdout, cursor::MoveTo(row_pos, j as u16), Print(cell)).unwrap();
                    }
                }
            }
            // render frame
            let (_, width) = get_cell(&0);
            let row_end_raw = ROW_START + (BACKGROUND_TOTAL_ROWS + 1) * width - 1;
            let col_end_raw = COL_START + BACKGROUND_TOTAL_COLS;
            {

                for i in ROW_START..=row_end_raw {
                    let text = get_text(&BORDER_HORI_CHAR.to_string(), &0);
                    execute!(stdout, cursor::MoveTo(i as u16, COL_START as u16), Print(&text)).unwrap();
                    execute!(stdout, cursor::MoveTo(i as u16, col_end_raw as u16), Print(&text)).unwrap();
                }
                for i in COL_START..=col_end_raw {
                    let text = get_text(&BORDER_VERT_CHAR.to_string(), &0);
                    execute!(stdout, cursor::MoveTo(ROW_START as u16, i as u16), Print(&text)).unwrap();
                    execute!(stdout, cursor::MoveTo(row_end_raw as u16, i as u16), Print(&text)).unwrap();
                }

                let text_corner_top_left = get_text(&CORNER_TOP_LEFT_CHAR.to_string(), &2);
                let text_corner_top_right = get_text(&CORNER_TOP_RIGHT_CHAR.to_string(), &2);
                let text_corner_bot_left = get_text(&CORNER_BOT_LEFT_CHAR.to_string(), &2);
                let text_corner_bot_right = get_text(&CORNER_BOT_RIGHT_CHAR.to_string(), &2);

                execute!(stdout, cursor::MoveTo(ROW_START as u16, COL_START as u16), Print(&text_corner_top_left)).unwrap();
                execute!(stdout, cursor::MoveTo(ROW_START as u16, col_end_raw as u16), Print(&text_corner_bot_left)).unwrap();
                execute!(stdout, cursor::MoveTo(row_end_raw as u16, COL_START as u16), Print(&text_corner_top_right)).unwrap();
                execute!(stdout, cursor::MoveTo(row_end_raw as u16, col_end_raw as u16), Print(&text_corner_bot_right)).unwrap();
            }
            // render title
            {
                let text_raw = format!("{} T E T I - R S {}", BORDER_VERT_CHAR, BORDER_VERT_CHAR);
                let text = get_text(&text_raw, &2);
                execute!(stdout, cursor::MoveTo(10, 0), Print(&text)).unwrap();
            }
        }

        pub fn render_update(&self, next_tetris: &Tetris, saved_tetris: &Option<Tetris>, score: &i32, cur_tetris: &Tetris, grid: &Grid) {
            let mut stdout = &self.stdout;
            // position grids
            let grid_start_row = ROW_START + PADDING;
            let main_grid_start_col = (COL_START + PADDING) * 2;
            let second_grid_start_col = COL_START + PADDING * 2 + MAIN_GRID_COLS;
            let second_grid_end_col = COL_START + PADDING * 2 + MAIN_GRID_COLS + SECND_GRID_COLS;
            // upcoming tetris
            let upcoming_end_row = grid_start_row + SECND_GRID_ROWS;
            for i in second_grid_start_col..=second_grid_end_col {
                for j in grid_start_row..upcoming_end_row {
                    let (cell, width) = get_cell(&0);
                    let row_pos = (width * i) as u16;
                    execute!(stdout, cursor::MoveTo(row_pos, j as u16), Print(cell)).unwrap();
                }
            }
            for row_col in next_tetris.get_tiles() {
                let new_row = row_col.row + grid_start_row as usize + 1;
                let new_col = row_col.col + second_grid_start_col as usize + 1;
                let (cell, width) = get_cell(&next_tetris.color);
                execute!(stdout, cursor::MoveTo((new_col * width as usize) as u16, new_row as u16), Print(cell)).unwrap();
            }
            // stored tetris
            let stored_start_row = upcoming_end_row + PADDING + 1;
            let stored_end_row = stored_start_row + SECND_GRID_ROWS;
            for i in second_grid_start_col..=second_grid_end_col {
                for j in stored_start_row..stored_end_row {
                    let (cell, width) = get_cell(&0);
                    let row_pos = (width * i) as u16;
                    execute!(stdout, cursor::MoveTo(row_pos, j as u16), Print(cell)).unwrap();
                }
            }
            if let Some(tet) = &saved_tetris {
                for row_col in tet.get_tiles() {
                    let new_row = row_col.row + stored_start_row as usize + 1;
                    let new_col = row_col.col + second_grid_start_col as usize + 1;
                    let (cell, width) = get_cell(&tet.color);
                    execute!(stdout, cursor::MoveTo((new_col * width as usize) as u16, new_row as u16), Print(cell)).unwrap();
                }
            }
            // score
            let score_start_row = stored_end_row + PADDING + 1;
            let score_end_row = score_start_row + SCORE_ROWS;
            for i in second_grid_start_col..=second_grid_end_col {
                for j in score_start_row..score_end_row {
                    let (cell, width) = get_cell(&0);
                    let row_pos = (width * i) as u16;
                    execute!(stdout, cursor::MoveTo(row_pos, j as u16), Print(cell)).unwrap();
                }
            }
            let score_preface = get_text(&"score:", &1);
            let score_string = get_text(&score.to_string(), &1);
            let (_, width) = get_cell(&0);
            let score_text_start_col_raw = (second_grid_start_col + 1) * width;
            execute!(stdout, cursor::MoveTo(score_text_start_col_raw as u16, score_start_row as u16), Print(score_preface)).unwrap();
            execute!(stdout, cursor::MoveTo(score_text_start_col_raw as u16, (score_start_row + 1) as u16), Print(score_string)).unwrap();
            
            execute!(stdout, cursor::MoveTo(main_grid_start_col as u16, grid_start_row as u16)).unwrap();
            let mut rendering_grid_vec = grid.grid_vec.clone();
            let shadow = cur_tetris.get_droped_tetris(&grid.grid_vec);
            for row_col in &shadow.get_poses() {
                *rendering_grid_vec.get_mut(row_col.row).unwrap().get_mut(row_col.col).unwrap() = 7;
            }
            for row_col in &cur_tetris.get_poses() {
                *rendering_grid_vec.get_mut(row_col.row).unwrap().get_mut(row_col.col).unwrap() = cur_tetris.color;
            }
            for (index, row) in rendering_grid_vec.iter().enumerate() {
                let mut row_string_vec: Vec<String> = vec![];
                for cell_color in row.iter() {
                    let (cell, _) = get_cell(cell_color);
                    row_string_vec.push(cell);
                }
                let row_string = row_string_vec.join("");
                execute!(stdout, cursor::MoveTo(main_grid_start_col as u16, index as u16 + 1), Print(row_string)).unwrap();
            }
        }
    }

    fn get_cell(cell: &usize) -> (String, i32) {
        let cell_uncolored = "███";
        let cell_out = match cell {
            // background
            0 => format!("{}", Colorize::white(cell_uncolored)),
            3 => format!("{}", Colorize::blue(cell_uncolored)),

            // tetris
            1 => format!("{}", Colorize::red(cell_uncolored)),
            2 => format!("{}", Colorize::green(cell_uncolored)),
            4 => format!("{}", Colorize::purple(cell_uncolored)),
            5 => format!("{}", Colorize::cyan(cell_uncolored)),
            6 => format!("{}", Colorize::bright_blue(cell_uncolored)),
            8 => format!("{}", Colorize::bright_red(cell_uncolored)),
            9 => format!("{}", Colorize::bright_green(cell_uncolored)),
            // tetris shadow
            7 => format!("{}", Stylize::dark_grey(cell_uncolored)),
            
            _ => format!("{}", Colorize::white(cell_uncolored)),
        };

        return (cell_out, 3)
    }

    fn get_text(text: &str, color: &usize) -> String {
        match color {
            // background
            0 => return format!("{}", Colorize::white(text).on_blue()),
            1 => return format!("{}", Colorize::black(text).on_white()),
            2 => return format!("{}", Colorize::white(text).on_blue().bold()),
            
            _ => return format!("{}", Colorize::white(text)),
        }
    }

    fn get_score(lines_cleared: i32) -> i32 {
        let base_multiplier = 1000; 
        let tetris_multiplier = 1.5;

        let mut score = (lines_cleared * base_multiplier) as f64;
        if lines_cleared >= 4 {
            score *= tetris_multiplier;
        }
        
        return score as i32
    }
}