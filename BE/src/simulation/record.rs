use serde::Serialize;
use super::map::entity::{Action};
use super::map::entity::carnivore::Carnivore;
use super::map::entity::herbivore::Herbivore;

#[derive(Clone)]
#[derive(Serialize)]
pub struct GenerationRecording {
    pub carnivores_at_start: Vec<Option<Carnivore>>,
    pub herbivores_at_start: Vec<Option<Herbivore>>,
    pub grass_at_start: Vec<(usize, usize)>,
    pub carnivore_records: Vec<Vec<AnimalRecord>>,
    pub herbivore_records: Vec<Vec<AnimalRecord>>,
    pub dead_grass: Vec<(usize, usize, usize)>, //Tick, x, y
}

impl GenerationRecording {
    pub fn new(carnivore_count: usize, herbivore_count: usize, grass_count: usize, carnivores: Vec<Option<Carnivore>>, herbivores: Vec<Option<Herbivore>>) -> GenerationRecording {
        GenerationRecording {
            carnivores_at_start: carnivores,
            herbivores_at_start: herbivores,
            grass_at_start: Vec::with_capacity(grass_count),
            carnivore_records: vec![Vec::new(); carnivore_count],
            herbivore_records: vec![Vec::new(); herbivore_count],
            dead_grass: Vec::with_capacity(grass_count),
        }
    }
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct AnimalRecord {
    pub energy: i32,
    pub action: Action,
    pub pos_x: usize,
    pub pos_y: usize,
}