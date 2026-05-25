mod simulation;

use simulation::Simulation;

fn main() {
    let mut sim: Simulation = Simulation::new();
    sim.simulate();
}
