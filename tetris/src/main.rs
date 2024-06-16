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
mod grid;
use crate::grid::grids;

fn main() {
    // init
    let mut grid = grids::Grid {
        grid_vec: vec![vec![0; 10]; 20],
    };
    let mut stdout = stdout();
    let mut cur_tetris = build::build_square_tetris(0, 0);
    let frame_time_millis = 10;
    let duration = Duration::from_millis(frame_time_millis as u64);
    let mut drop_timer = 0;
    let drop_time_millis = 100;
    enable_raw_mode().unwrap();

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    // render border
    {
        for i in 0..=15 {
            for j in 0..=21 {
                let cell = get_cell(&3);
                let row_pos = (2 * i) as u16;
                execute!(stdout, cursor::MoveTo(row_pos, j), Print(cell)).unwrap();
            }
        }
    }
    loop {
        // render (13, 1) to (15, 2) upcoming and stored.
        {
            for i in 12..=14 {
                for j in 1..=2 {
                    let cell = get_cell(&0);
                    let row_pos = (2 * i) as u16;
                    execute!(stdout, cursor::MoveTo(row_pos, j), Print(cell)).unwrap();
                }
            }
            for i in 12..=14 {
                for j in 4..=5 {
                    let cell = get_cell(&0);
                    let row_pos = (2 * i) as u16;
                    execute!(stdout, cursor::MoveTo(row_pos, j), Print(cell)).unwrap();
                }
            }
        }
        // render (1,1) to (11,21) is tetris grid.
        {
            execute!(stdout, cursor::MoveTo(1, 1)).unwrap();
            let mut rendering_grid_vec = grid.grid_vec.clone();
            for pos in &cur_tetris.poses {
                *rendering_grid_vec.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap() = cur_tetris.color;
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
        let mut drop = false;
        // step drop;
        drop_timer -= frame_time_millis;
        if drop_timer <= 0 {
            shift.0 = -1;
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
                }) => shift.1 = 1,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => shift.1 = -1,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('j'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => shift.0 = -1,
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => drop = true,
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
            cur_tetris = build::build_square_tetris(0, 0);
        }
        else if !cur_tetris.move_tetris(&grid.grid_vec, &shift) {
            grid.apply_tetris(&cur_tetris);
            cur_tetris = build::build_square_tetris(0, 0);
        }
        std::thread::sleep(duration);
    }

    disable_raw_mode().unwrap();
}

fn get_cell(cell: &usize) -> String {
    let cell_uncolored = "██";
    match cell {
        0 => return format!("{}", cell_uncolored.white()),
        1 => return format!("{}", cell_uncolored.red()),
        2 => return format!("{}", cell_uncolored.green()),
        3 => return format!("{}", cell_uncolored.blue()),
        4 => return format!("{}", cell_uncolored.purple()),
        _ => return format!("{}", cell_uncolored.white()),
    }
}

