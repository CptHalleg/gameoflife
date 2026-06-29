use crate::conway::ConwayAutomaton;
use crate::simulation::Simulation;

mod conway;
mod grid;
mod parsing;
mod simulation;

fn main() {
    let h: usize = 50;
    let w: usize = 100;
    let automation = ConwayAutomaton;

    let mut simulation = Simulation::new(w, h, automation);
    simulation.init_grid();
    simulation.simulate_loop();
    println!("end state:");
    simulation.print();
}
