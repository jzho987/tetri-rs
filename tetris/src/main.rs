extern crate colored;
use colored::Colorize;

use std::env::current_exe;
use std::time::Duration;
use std::io;
extern crate futures_timer;
use futures_timer::Delay;

#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind, KeyEventState};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Write};

fn main() {
    // init
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 10]; 20];
    let mut stdout = stdout();
    let mut cur_blocks = vec![(0, 1), (0, 2), (0, 3), (1, 2)];
    let mut cur_tetris_type = 1;
    let duration = Duration::from_millis(20);

    enable_raw_mode().unwrap();

    loop {
        let mut render_grid = grid.clone();
        for pos in &cur_blocks {
            *render_grid.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap() = cur_tetris_type;
        }

        for item in render_grid.iter().enumerate() {
            let mut line_unjoined: Vec<String> = vec![];
            for cell in item.1.iter() {
                line_unjoined.push(get_cell(cell));
            }
            let line = line_unjoined.join("");
            execute!(stdout, cursor::MoveTo(0, item.0 as u16), Print(line)).unwrap();
        }

        let mut shift = (-1_i32, 0_i32);
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
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => break,
                _ => (),
            }
        };
        
        let new_blocks = move_tetris(&cur_blocks, &grid, &shift);
        if cur_blocks == new_blocks {
            for pos in &cur_blocks {
                *grid.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap() = cur_tetris_type;
            }
            cur_blocks = vec![(0, 1), (0, 2), (0, 3), (1, 2)];
        } else {
            cur_blocks = new_blocks;
        }

        std::thread::sleep(duration);
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
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
