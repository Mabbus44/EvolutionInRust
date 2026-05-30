pub mod config;
mod map;
mod record;

use map::Map;
use config::*;
use crate::simulation::record::GenerationRecording;

pub struct Simulation {
    map: Map,
    config: SimulationConfig,
    generation: usize,
    recording: Vec<GenerationRecording>,
}

impl Simulation {
    pub fn new(config: SimulationConfig) -> Simulation {
        Simulation {
            map: Map::new(config.map_config.clone(), config.generation_config.clone(), config.mutation_config.clone()),
            config,
            generation: 0,
            recording: Vec::new(),
        }
    }

    pub fn simulate(&mut self){
        while self.generation < self.config.generation_config.max_generation_count as usize {
            if self.generation > 0 {
                self.map.start_new_generation();
            }
            while !self.map.handle_and_return_generation_over() {
                self.map.tick();
            }
            if self.config.map_config.record {
                self.recording.push(self.map.get_recording().unwrap());
            }
            self.generation += 1;
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        self.map.to_string()
    }

    pub fn get_recording_as_json(&self) -> String {
        serde_json::json!(self.recording).to_string()
    }
}