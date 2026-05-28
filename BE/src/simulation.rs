pub mod config;
mod map;
mod record;

use map::Map;
use config::*;

pub struct Simulation {
    map: Map
}

impl Simulation {
    pub fn new(config: SimulationConfig) -> Simulation {
        Simulation {
            map: Map::new(config.map_config, config.generation_config.clone(), config.mutation_config.clone()),
        }
    }

    pub fn simulate(&mut self){
        //print!("{}", self.to_string());
        while !self.map.is_simulation_over() {
            while !self.map.set_and_return_generation_over() {
                self.map.tick();
            }
            /*println!();
            println!();
            println!("Generation over after {} steps", self.map.get_tick_count());
            println!("********************************************************");
            println!();
            println!();
            print!("{}", self.to_string());*/
            self.map.start_new_generation();
        }
        //println!("************simulation over**************);");
    }
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        self.map.to_string()
    }

    pub fn get_recording_as_json(&self) -> String {
        let ret = serde_json::json!({
            "generations": self.map.get_recording()
        }).to_string();
        ret
    }
}