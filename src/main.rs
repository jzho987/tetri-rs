extern crate colored;
use colored::Colorize;

extern crate futures_timer;
use std::time::Duration;

#[macro_use]
extern crate crossterm;
use crossterm::cursor;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind, KeyEventState};
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use std::io::stdout;

mod models;
use crate::models::tetris::Tetris;
use crate::models::grid::Grid;

mod builder;
use crate::builder::build;

// TODO: add title screen and score etc.
// TODO: make the render better.

const CORNER_TOP_LEFT_CHAR: char = '╔';
const CORNER_TOP_RIGHT_CHAR: char = '╗';
const CORNER_BOT_LEFT_CHAR: char = '╚';
const CORNER_BOT_RIGHT_CHAR: char = '╝';
const BORDER_VERT_CHAR: char = '║';
const BORDER_HORI_CHAR: char = '═';

fn main() {
    // init
    let mut grid = Grid {
        grid_vec: vec![vec![0; 10]; 20],
    };
    let mut stdout = stdout();
    let mut cur_tetris = build::build_random_tetris(0, 0);
    let mut next_tetris = build::build_random_tetris(0, 0);
    let mut saved_tetris: Option<Tetris> = None;

    // init timers
    let frame_time_millis = 10;
    let duration = Duration::from_millis(frame_time_millis as u64);
    let mut drop_timer = 0;
    let drop_time_millis = 300;

    // init score
    let mut score = 0;

    // prep render
    enable_raw_mode().unwrap();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    // render background
    {
        for i in 0..=18 {
            for j in 0..=23 {
                let cell = get_cell(&3);
                let row_pos = (2 * i) as u16;
                execute!(stdout, cursor::MoveTo(row_pos, j), Print(cell)).unwrap();
            }
        }
    }
    // render frame
    {
        let row_start = 0;
        let row_end = 37;
        let col_start = 0;
        let col_end = 23;

        for i in row_start..=row_end {
            let text = get_text(&BORDER_HORI_CHAR.to_string(), &0);
            execute!(stdout, cursor::MoveTo(i, col_start), Print(&text)).unwrap();
            execute!(stdout, cursor::MoveTo(i, col_end), Print(&text)).unwrap();
        }
        for i in col_start..=col_end {
            let text = get_text(&BORDER_VERT_CHAR.to_string(), &0);
            execute!(stdout, cursor::MoveTo(row_start, i), Print(&text)).unwrap();
            execute!(stdout, cursor::MoveTo(row_end, i), Print(&text)).unwrap();
        }

        let text_corner_top_left = get_text(&CORNER_TOP_LEFT_CHAR.to_string(), &2);
        let text_corner_top_right = get_text(&CORNER_TOP_RIGHT_CHAR.to_string(), &2);
        let text_corner_bot_left = get_text(&CORNER_BOT_LEFT_CHAR.to_string(), &2);
        let text_corner_bot_right = get_text(&CORNER_BOT_RIGHT_CHAR.to_string(), &2);

        execute!(stdout, cursor::MoveTo(row_start, col_start), Print(&text_corner_top_left)).unwrap();
        execute!(stdout, cursor::MoveTo(row_start, col_end), Print(&text_corner_bot_left)).unwrap();
        execute!(stdout, cursor::MoveTo(row_end, col_start), Print(&text_corner_top_right)).unwrap();
        execute!(stdout, cursor::MoveTo(row_end, col_end), Print(&text_corner_bot_right)).unwrap();
    }
    // render title
    {
        let text_raw = format!("{} T E T I - R S {}", BORDER_VERT_CHAR, BORDER_VERT_CHAR);
        let text = get_text(&text_raw, &2);
        execute!(stdout, cursor::MoveTo(10, 0), Print(&text)).unwrap();
    }

    loop {
        // render (13, 1) to (15, 2) upcoming and stored.
        {
            for i in 12..=17 {
                for j in 1..=4 {
                    let cell = get_cell(&0);
                    let row_pos = (2 * i) as u16;
                    execute!(stdout, cursor::MoveTo(row_pos, j), Print(cell)).unwrap();
                }
            }
            for row_col in next_tetris.get_tiles() {
                let new_row = row_col.row + 2;
                let new_col = row_col.col + 13;
                execute!(stdout, cursor::MoveTo((new_col * 2) as u16, new_row as u16), Print(get_cell(&next_tetris.color))).unwrap();
            }
            for i in 12..=17 {
                for j in 6..=9 {
                    let cell = get_cell(&0);
                    let row_pos = (2 * i) as u16;
                    execute!(stdout, cursor::MoveTo(row_pos, j), Print(cell)).unwrap();
                }
            }
            if let Some(tet) = &saved_tetris {
                for row_col in tet.get_tiles() {
                    let new_row = row_col.row + 7;
                    let new_col = row_col.col + 13;
                    execute!(stdout, cursor::MoveTo((new_col * 2) as u16, new_row as u16), Print(get_cell(&tet.color))).unwrap();
                }
            }
        }
        // render score
        {

            for i in 12..=17 {
                for j in 11..=12 {
                    let cell = get_cell(&0);
                    let row_pos = (2 * i) as u16;
                    execute!(stdout, cursor::MoveTo(row_pos, j), Print(cell)).unwrap();
                }
            }
            let score_preface = get_text(&"score:", &1);
            let score_string = get_text(&score.to_string(), &1);
            execute!(stdout, cursor::MoveTo(13 * 2, 11), Print(score_preface)).unwrap();
            execute!(stdout, cursor::MoveTo(13 * 2, 12), Print(score_string)).unwrap();
            
        }
        // render (1,1) to (11,21) is tetris grid.
        {
            execute!(stdout, cursor::MoveTo(1, 1)).unwrap();
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
                for cell in row.iter() {
                    row_string_vec.push(get_cell(cell));
                }
                let row_string = row_string_vec.join("");
                execute!(stdout, cursor::MoveTo(2, index as u16 + 1), Print(row_string)).unwrap();
            }
        }

        let mut shift = (0_i32, 0_i32);
        let mut spin = 0;
        let mut drop = false;
        let mut save = false;
        // step drop;
        drop_timer -= frame_time_millis;
        if drop_timer <= 0 {
            shift.0 = 1;
            drop_timer = drop_time_millis;
        }
        // get io and wait;
        if poll(duration).unwrap() {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('h'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => shift.1 = -1,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => shift.1 = 1,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('j'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => shift.0 += 1,
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => drop = true,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('k'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => spin = -1,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('z'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => spin = 1,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => save = true,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => break,
                _ => (),
            }
        };
        if drop {
            cur_tetris.drop_tetris(&grid.grid_vec);

            let lines_cleared = grid.apply_tetris(&cur_tetris);
            score += get_score(lines_cleared);

            cur_tetris = next_tetris;
            next_tetris = build::build_random_tetris(0, 0);
        }
        else if spin != 0 {
            cur_tetris.try_spin_tetris(spin, &grid);
        }
        else if save {
            match saved_tetris {
                Some(tet) => {
                    cur_tetris.reset_tetris();
                    saved_tetris = Some(cur_tetris);
                    cur_tetris = tet;
                },
                None => {
                    cur_tetris.reset_tetris();
                    saved_tetris = Some(cur_tetris);
                    cur_tetris = next_tetris;
                    next_tetris = build::build_random_tetris(0, 0);
                },
            };
        }
        else if !cur_tetris.try_move_or_set_tetris(&grid.grid_vec, &shift) {
            let lines_cleared = grid.apply_tetris(&cur_tetris);
            score += get_score(lines_cleared);

            cur_tetris = next_tetris;
            next_tetris = build::build_random_tetris(0, 0);
        }
    }

    disable_raw_mode().unwrap();
}

fn get_cell(cell: &usize) -> String {
    let cell_uncolored = "██";
    match cell {
        // background
        0 => return format!("{}", Colorize::white(cell_uncolored)),
        3 => return format!("{}", Colorize::blue(cell_uncolored)),

        // tetris
        1 => return format!("{}", Colorize::red(cell_uncolored)),
        2 => return format!("{}", Colorize::green(cell_uncolored)),
        4 => return format!("{}", Colorize::purple(cell_uncolored)),
        5 => return format!("{}", Colorize::cyan(cell_uncolored)),
        6 => return format!("{}", Colorize::bright_blue(cell_uncolored)),
        // tetris shadow
        7 => return format!("{}", Stylize::dark_grey(cell_uncolored)),
        
        _ => return format!("{}", Colorize::white(cell_uncolored)),
    }
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