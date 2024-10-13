extern crate colored;

extern crate futures_timer;
use std::time::Duration;

#[macro_use]
extern crate crossterm;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind, KeyEventState};
use crossterm::terminal::disable_raw_mode;

mod models;
use crate::models::tetris::Tetris;
use crate::models::grid::Grid;

mod builder;
use crate::builder::build;

mod renderer;
use renderer::render::Renderer;

// TODO: add title screen and score etc.
// TODO: make the render better.

fn main() {
    // init
    let mut grid = Grid {
        grid_vec: vec![vec![0; 10]; 20],
    };
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

    let renderer = Renderer::new();
    renderer.render_background();

    loop {
        renderer.render_update(&next_tetris, &saved_tetris, &score, &cur_tetris, &grid);

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

fn get_score(lines_cleared: i32) -> i32 {
    let base_multiplier = 1000; 
    let tetris_multiplier = 1.5;

    let mut score = (lines_cleared * base_multiplier) as f64;
    if lines_cleared >= 4 {
        score *= tetris_multiplier;
    }
    
    return score as i32
}

