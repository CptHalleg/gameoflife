use crate::grid::Grid;
use crate::grid::GridPosition;
use crate::grid::NEIGHBOUR_OFFSETS;
use crate::simulation::CellularAutomaton;
use std::io::Stdout;
use std::io::Write;
pub struct ConwayAutomaton;

impl CellularAutomaton<bool> for ConwayAutomaton {
    fn print_cell(&self, out: &mut Stdout, value: bool) {
        if value {
            write!(out, "X").unwrap();
        } else {
            write!(out, " ").unwrap()
        }
    }

    fn parse_char(&self, ch: &char) -> bool {
        ch.is_alphabetic()
    }

    fn update_cell(&self, pos: GridPosition, old_grid: &Grid<bool>) -> bool {
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
        rules(old_grid[pos], count)
    }
}

fn rules(current_value: bool, neighbours: u8) -> bool {
    match neighbours {
        2 => current_value,
        3 => true,
        _ => false,
    }
}
