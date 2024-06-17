extern crate colored;
use colored::Colorize;

extern crate futures_timer;
use std::time::Duration;

#[macro_use]
extern crate crossterm;
use crossterm::cursor;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind, KeyEventState};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use std::io::stdout;

mod builder;
use crate::builder::build;
use crate::builder::tetris::Tetris;
mod grid;
use crate::grid::grids;

// TODO: fix the spin to work with collision avoidance.
// TODO: fix the save system.

// TODO: improve IO and stutter.
// TODO: add title screen and score etc.
// TODO: make the render better.

fn main() {
    // init
    let mut grid = grids::Grid {
        grid_vec: vec![vec![0; 10]; 20],
    };
    let mut stdout = stdout();
    let mut cur_tetris = build::build_random_tetris(0, 0);
    let mut next_tetris = build::build_random_tetris(0, 0);
    let mut saved_tetris: Option<Tetris> = None;
    let frame_time_millis = 10;
    let duration = Duration::from_millis(frame_time_millis as u64);
    let mut drop_timer = 0;
    let drop_time_millis = 100;
    enable_raw_mode().unwrap();

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    // render border
    {
        for i in 0..=18 {
            for j in 0..=23 {
                let cell = get_cell(&3);
                let row_pos = (2 * i) as u16;
                execute!(stdout, cursor::MoveTo(row_pos, j), Print(cell)).unwrap();
            }
        }
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
            for row_col in &next_tetris.tiles {
                let new_row = row_col.row + 13;
                let new_col = row_col.col + 2;
                execute!(stdout, cursor::MoveTo((new_row * 2) as u16, new_col as u16), Print(get_cell(&next_tetris.color))).unwrap();
            }
            for i in 12..=17 {
                for j in 6..=9 {
                    let cell = get_cell(&0);
                    let row_pos = (2 * i) as u16;
                    execute!(stdout, cursor::MoveTo(row_pos, j), Print(cell)).unwrap();
                }
            }
            if let Some(tet) = &saved_tetris {
                for row_col in &tet.tiles {
                    let new_row = row_col.row + 13;
                    let new_col = row_col.col + 7;
                    execute!(stdout, cursor::MoveTo((new_row * 2) as u16, new_col as u16), Print(get_cell(&tet.color))).unwrap();
                }
            }
        }
        // render (1,1) to (11,21) is tetris grid.
        {
            execute!(stdout, cursor::MoveTo(1, 1)).unwrap();
            let mut rendering_grid_vec = grid.grid_vec.clone();
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
            // shift.0 = 1;
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
            grid.apply_tetris(&cur_tetris);
            cur_tetris = next_tetris;
            next_tetris = build::build_random_tetris(0, 0);
        }
        else if spin != 0 {
            cur_tetris.spin_tetris(spin);
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
        else if !cur_tetris.move_tetris(&grid.grid_vec, &shift) {
            grid.apply_tetris(&cur_tetris);
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
        0 => return format!("{}", cell_uncolored.white()),
        3 => return format!("{}", cell_uncolored.blue()),

        // tetris
        1 => return format!("{}", cell_uncolored.red()),
        2 => return format!("{}", cell_uncolored.green()),
        4 => return format!("{}", cell_uncolored.purple()),
        5 => return format!("{}", cell_uncolored.cyan()),
        6 => return format!("{}", cell_uncolored.bright_blue()),
        
        _ => return format!("{}", cell_uncolored.white()),
    }
}
