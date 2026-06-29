use crate::grid::Grid;
use crate::grid::GridPosition;
use crate::grid::NEIGHBOUR_OFFSETS;
use crate::parsing::parse_chars;
use crate::simulation::CellularAutomaton;
use std::env;
use std::fs;
use std::io::Stdout;
use std::io::Write;
pub struct ConwayAutomaton;

impl CellularAutomaton<bool> for ConwayAutomaton {
    fn print(&self, out: &mut Stdout, value: bool) {
        if value {
            write!(out, "X").unwrap();
        } else {
            write!(out, " ").unwrap()
        }
    }

    fn init_grid(&self, grid: &mut Grid<bool>) {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            eprintln!("Usage: {} <filename>", args[0]);
            return;
        }

        let filename = &args[1];
        let contents = fs::read_to_string(filename).expect("Failed to read file");
        let parsed = parse_chars(&contents).unwrap_or_else(|err| {
            panic!("parsing failed: {}", err);
        });
        let load_pos = grid.create_position(15, 30);
        grid.load_grid(parsed, load_pos.unwrap());
    }

    fn update_grid(&self, pos: GridPosition, old_grid: &Grid<bool>) -> bool {
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
}

fn update_cell(current_value: bool, neighbours: u8) -> bool {
    match neighbours {
        2 => current_value,
        3 => true,
        _ => false,
    }
}
