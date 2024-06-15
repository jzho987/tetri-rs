extern crate colored;
use colored::Colorize;

use std::time::Duration;
extern crate futures_timer;

#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind, KeyEventState};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::stdout;

mod builder;
use crate::builder::tetris::Tetris;
use crate::builder::build;

fn main() {
    // init
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 10]; 20];
    let mut stdout = stdout();
    let mut cur_tetris = build::build_tee_tetris(0, 0);
    let duration = Duration::from_millis(20);
    let fall_speed = Duration::from_secs(1);
    let mut fall_counter = fall_speed.clone();

    enable_raw_mode().unwrap();

    loop {
        // render current grid state;
        {
            execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
            let mut rendering_grid = grid.clone();
            for pos in &cur_tetris.poses {
                *rendering_grid.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap() = cur_tetris.color as usize;
            }
            for (index, row) in rendering_grid.iter().enumerate() {
                let mut row_string_vec: Vec<String> = vec![];
                for cell in row.iter() {
                    row_string_vec.push(get_cell(cell));
                }
                let row_string = row_string_vec.join("");
                execute!(stdout, cursor::MoveTo(0, index as u16), Print(row_string)).unwrap();
            }
        }

        let mut shift = (0_i32, 0_i32);
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
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => break,
                _ => (),
            }
        };
        // apply move
        let new_poses = move_tetris(&cur_tetris.poses, &grid, &shift);
        if cur_tetris.poses == new_poses {
            for pos in &cur_tetris.poses {
                *grid.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap() = cur_tetris.color as usize;
            }
            cur_tetris = build::build_square_tetris(0, 0);
        } else {
            cur_tetris.poses = new_poses;
        }
        std::thread::sleep(duration);
    }
    
    disable_raw_mode().unwrap();
}

fn get_cell(cell: &usize) -> String {
    let cell_uncolored = "███";
    match cell {
        0 => return format!("{}", cell_uncolored.white()),
        1 => return format!("{}", cell_uncolored.red()),
        2 => return format!("{}", cell_uncolored.green()),
        3 => return format!("{}", cell_uncolored.blue()),
        4 => return format!("{}", cell_uncolored.purple()),
        _ => return format!("{}", cell_uncolored.white()),
    }
}

fn move_tetris(blocks: &Vec<(usize, usize)>, grid: &Vec<Vec<usize>>, direction: &(i32, i32)) -> Vec<(usize, usize)> {
    let mut after_position = vec![];
    let col_count = grid.get(0).unwrap().len() as i32;
    let row_count = grid.len() as i32;
    for pos in blocks {
        let after_col = pos.1 as i32 - direction.1;
        let after_row = pos.0 as i32 - direction.0;

        if after_col < 0 || after_col >= col_count {
            after_position = blocks.clone();
            break;
        }

        if after_row < 0 || after_row >= row_count {
            after_position = blocks.clone();
            break;
        }

        if *grid
            .get(after_row as usize).unwrap()
            .get(after_col as usize).unwrap()
            != 0 as usize {
            
            after_position = blocks.clone();
            break;
        }

        after_position.push((after_row as usize, after_col as usize))
    }

    return after_position;
}
