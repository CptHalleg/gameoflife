mod grid;
use crate::grid::NEIGHBOUR_OFFSETS;
use crossterm::cursor::MoveTo;
use crossterm::event::poll;
use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use crossterm::terminal::*;
use crossterm::*;
use grid::Grid;
use grid::GridPosition;
use std::io::Write;
use std::io::stdout;
use std::thread;
use std::time::Duration;
use std::time::Instant;

const H: usize = 50;
const W: usize = 100;

fn main() {
    println!("Okay lets go!");
    let mut current_grid: Grid<bool> = Grid::new(W, H);
    let mut new_grid: Grid<bool> = Grid::new(W, H);

    let pulsar = [
        (-6, -4),
        (-6, -3),
        (-6, -2),
        (-4, -6),
        (-3, -6),
        (-2, -6),
        (-4, -1),
        (-3, -1),
        (-2, -1),
        (-4, 1),
        (-3, 1),
        (-2, 1),
        (-1, -4),
        (-1, -3),
        (-1, -2),
        (1, -4),
        (1, -3),
        (1, -2),
        (2, -6),
        (3, -6),
        (4, -6),
        (2, -1),
        (3, -1),
        (4, -1),
        (2, 1),
        (3, 1),
        (4, 1),
        (4, 6),
        (3, 6),
        (2, 6),
        (6, -4),
        (6, -3),
        (6, -2),
        (-6, 2),
        (-6, 3),
        (-6, 4),
        (-4, 6),
        (-3, 6),
        (-2, 6),
        (-1, 4),
        (-1, 3),
        (-1, 2),
        (1, 4),
        (1, 3),
        (1, 2),
        (2, 6),
        (3, 6),
        (4, 6),
    ];

    //delta 35.45958638191223
    //delta 3.9695897102355957

    set_pattern(&mut current_grid, 10, 10, &pulsar);

    let glider = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];

    set_pattern(&mut current_grid, 5, 5, &glider);
    let frame_time = Duration::from_millis(100);

    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();
    loop {
        let start = Instant::now();
        Grid::update_chunks(&mut new_grid, &current_grid, 16, update_grid);
        print(&new_grid, start.elapsed().as_secs_f64());

        std::mem::swap(&mut new_grid, &mut current_grid);
        thread::sleep(Duration::from_millis(100));

        // update / draw

        if poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                if key_event.code == KeyCode::Char('c')
                    && key_event.modifiers.contains(KeyModifiers::CONTROL)
                {
                    break;
                }
            }
        }

        let elapsed = start.elapsed();
        if elapsed < frame_time {
            thread::sleep(frame_time - elapsed);
        }
    }

    execute!(stdout(), LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
    println!("restored terminal")
}

pub fn print(grid: &Grid<bool>, delta: f64) {
    let mut out = stdout();
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    if true {
        write!(out, "|").unwrap();
        for _ in 0..grid.width {
            write!(out, "-").unwrap();
        }
        for pos in 0..grid.count {
            if pos % grid.width == 0 {
                write!(out, "|\r\n|").unwrap();
            }
            if grid.vec[pos] {
                write!(out, "X").unwrap();
            } else {
                write!(out, " ").unwrap()
            }
        }
        write!(out, "|\r\n|").unwrap();
        for _ in 0..grid.width {
            write!(out, "-").unwrap();
        }
        write!(out, "|").unwrap();
    }
    write!(out, "\r\ndelta {}", delta).unwrap();
    stdout().flush().unwrap();
}

fn update_grid(pos: GridPosition, old_grid: &Grid<bool>) -> bool {
    let mut count = 0;
    for off in NEIGHBOUR_OFFSETS {
        match old_grid.add_offset_to_position(&pos, &off) {
            Some(new_pos) => {
                if old_grid[new_pos] {
                    count += 1;
                }
            }
            None => (),
        }
    }
    update_cell(old_grid[pos], count)
}

fn set_pattern(
    grid: &mut Grid<bool>,
    origin_row: usize,
    origin_col: usize,
    offsets: &[(isize, isize)],
) {
    for &(dr, dc) in offsets {
        let row = (origin_row as isize + dr) as usize;
        let col = (origin_col as isize + dc) as usize;
        let pos = grid.create_position(row, col);
        grid[pos] = true;
    }
}

fn update_cell(current_value: bool, neighbours: u8) -> bool {
    match neighbours {
        2 => current_value,
        3 => true,
        _ => false,
    }
}
