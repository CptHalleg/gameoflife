use crate::grid::Grid;
use crate::grid::GridPosition;
use crossterm::cursor::MoveTo;
use crossterm::event::poll;
use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use crossterm::terminal::*;
use crossterm::*;
use std::io::Stdout;
use std::io::Write;
use std::io::stdout;
use std::thread;
use std::time::Duration;
use std::time::Instant;

pub struct Simulation<T, S>
where
    T: Send + Sync + Copy + Default,
    S: CellularAutomaton<T>,
{
    grid: Grid<T>,
    automaton: S,
}

impl<T, S> Simulation<T, S>
where
    T: Send + Sync + Copy + Default,
    S: CellularAutomaton<T>,
{
    pub fn new(width: usize, height: usize, automaton: S) -> Simulation<T, S> {
        Simulation {
            grid: Grid::new(width, height),
            automaton: automaton,
        }
    }

    pub fn parse_string(&mut self, string: &str, pos: GridPosition) {
        let vec: Vec<Vec<T>> = string
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| self.automaton.parse_char(&ch))
                    .collect()
            })
            .collect();

        self.grid.load_grid(vec, pos);
    }

    pub fn simulate_loop(&mut self) {
        let mut old_grid: Grid<T> = self.grid.clone();

        let frame_time = Duration::from_millis(100);

        println!("Okay lets go!");
        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen).unwrap();
        loop {
            let start = Instant::now();

            Self::update_chunks(&mut self.grid, &old_grid, 16, &self.automaton);
            execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
            self.print();
            std::mem::swap(&mut self.grid, &mut old_grid);

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
        stdout().flush().unwrap();
        println!("calm down!")
    }

    pub fn print(&self) {
        let mut out = stdout();
        if true {
            write!(out, "|").unwrap();
            for _ in 0..self.grid.width {
                write!(out, "-").unwrap();
            }
            for pos in 0..self.grid.count {
                if pos % self.grid.width == 0 {
                    write!(out, "|\r\n|").unwrap();
                }

                self.automaton.print_cell(&mut out, self.grid.vec[pos]);
            }
            write!(out, "|\r\n|").unwrap();
            for _ in 0..self.grid.width {
                write!(out, "-").unwrap();
            }
            write!(out, "|").unwrap();
        }
        write!(out, "|\r\n|").unwrap();
        stdout().flush().unwrap();
    }

    fn update_chunks(
        new_grid: &mut Grid<T>,
        old_grid: &Grid<T>,
        chunk_count: usize,
        automaton: &S,
    ) {
        let chunk_size = new_grid.count / chunk_count;

        thread::scope(|s| {
            for (chunk_index, chunk) in new_grid.vec.chunks_mut(chunk_size).enumerate() {
                s.spawn(move || {
                    let chunk_start_pos = chunk_index * chunk_size;
                    for (cell_index, x) in chunk.iter_mut().enumerate() {
                        *x = automaton.update_cell(
                            old_grid
                                .create_position_index(chunk_start_pos + cell_index)
                                .unwrap(),
                            old_grid,
                        );
                    }
                });
            }
        });
    }
}

pub trait CellularAutomaton<T>
where
    T: Send + Sync + Copy + Default,
    Self: Sync,
{
    fn print_cell(&self, out: &mut Stdout, value: T);
    fn parse_char(&self, ch: &char) -> T;
    fn update_cell(&self, pos: GridPosition, old_grid: &Grid<T>) -> T;
}
