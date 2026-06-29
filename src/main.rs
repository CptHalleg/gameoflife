use crate::conway::ConwayAutomaton;
use crate::grid::ORIGIN_POSITION;
use crate::simulation::Simulation;
use std::env;
use std::fs;

mod conway;
mod grid;
mod parsing;
mod simulation;

fn main() {
    let h: usize = 50;
    let w: usize = 100;
    let automation = ConwayAutomaton;

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut simulation = Simulation::new(w, h, automation);
    simulation.parse_string(&contents, ORIGIN_POSITION);
    simulation.simulate_loop();
    println!("end state:");
    simulation.print();
}
