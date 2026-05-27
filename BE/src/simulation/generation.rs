#[derive(Clone)]
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

