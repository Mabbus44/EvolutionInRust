use serde::Deserialize;

#[derive(Clone)]
#[derive(Deserialize)]
pub struct SimulationConfig {
    pub map_config: MapConfig,
    pub generation_config: GenerationConfig,
    pub mutation_config: MutationConfig
}

#[derive(Clone)]
#[derive(Deserialize)]
pub struct MapConfig {
    pub carnivore_count: u32,
    pub herbivore_count: u32,
    pub grass_count: u32,
    pub neuron_count: usize,
    pub neuron_layer_count: usize,
    pub sense_radius: usize,
    pub carnivore_max_energy: u32,
    pub herbivore_max_energy: u32,
    pub size_x: usize,
    pub size_y: usize,
    pub record: bool,
}
#[derive(Clone)]
#[derive(Deserialize)]
pub struct GenerationConfig {
    pub max_generation_count: u32,
    pub max_ticks_per_generation: i32,
    pub carnivore_count: i32,
    pub herbivore_count: i32,
    pub grass_count: i32,
    pub best_carnivore_count: u32,
    pub best_herbivore_count: u32,
    pub all_entities_must_be_under_min_levels: bool,
}

#[derive(Clone)]
#[derive(Deserialize)]
pub struct MutationConfig {
    pub mutation_chance: f64,
    pub max_mutation_amount: f64
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            // Standard
            map_config: MapConfig {
                carnivore_count: 5,
                herbivore_count: 15,
                grass_count: 100,
                neuron_count: 10,
                neuron_layer_count: 2,
                sense_radius: 4,
                carnivore_max_energy: 100,
                herbivore_max_energy: 50,
                size_x: 200,
                size_y: 100,
                record: true,
            },
            generation_config: GenerationConfig {
                max_generation_count: 50,
                max_ticks_per_generation: 1000,
                carnivore_count: 2,
                herbivore_count: 2,
                grass_count: -1,
                best_carnivore_count: 2,
                best_herbivore_count: 2,
                all_entities_must_be_under_min_levels: true,
            },
            // Minimal
            /*map: MapRequest {
                carnivore_count: 2,
                herbivore_count: 2,
                grass_count: 3,
                neuron_count: 3,
                neuron_layer_count: 2,
                sense_radius: 1,
                carnivore_max_energy: 100,
                herbivore_max_energy: 50,
                size_x: 200,
                size_y: 100,
                record: true,
            },
            generation: GenerationRequest {
                max_generation_count: 1,
                max_ticks_per_generation: 10,
                carnivore_count: -1,
                herbivore_count: -1,
                grass_count: -1,
                best_carnivore_count: 2,
                best_herbivore_count: 2,
                all_entities_must_be_under_min_levels: true,
            },*/
            mutation_config: MutationConfig {
                mutation_chance: 0.01,
                max_mutation_amount: 0.3,
            },
        }
    }
}