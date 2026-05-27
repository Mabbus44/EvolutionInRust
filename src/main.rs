mod simulation;

use simulation::Simulation;
use simulation::map::MapConfig;
use simulation::generation::GenerationConfig;
use simulation::mutation::MutationConfig;

fn main() {
    let map_config = MapConfig {
        carnivore_count: 2,
        herbivore_count: 5,
        grass_count: 20,
        neuron_count: 10,
        neuron_layer_count: 2,
        sense_radius: 4,
        carnivore_max_energy: 5,
        herbivore_max_energy: 5,
        size_x: 20,
        size_y: 10,
        record: true,
    };
    let generation_config = GenerationConfig {
        max_generation_count: 4,
        max_ticks_per_generation: 10000,
        carnivore_count: -1,
        herbivore_count: 4,
        grass_count: -1,
        best_carnivore_count: 2,
        best_herbivore_count: 3,
        all_entities_must_be_under_min_levels: false
    };
    let mutation_config = MutationConfig {
        mutation_chance: 0.01,
        max_mutation_amount: 0.3

    };
    let mut sim: Simulation = Simulation::new(map_config, generation_config, mutation_config);
    sim.simulate();
    std::fs::write("temp.txt", sim.get_recording_as_json())
    .expect("Failed to write temp.txt");
}
