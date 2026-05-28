pub mod map;
pub mod generation;
pub mod mutation;
mod record;

use map::Map;
use map::MapConfig;
use generation::GenerationConfig;
use mutation::MutationConfig;

pub struct Simulation {
    map: Map
}

impl Simulation {
    pub fn new(map_config: MapConfig, generation_config: GenerationConfig, mutation_config: MutationConfig) -> Simulation {
        Simulation {
            map: Map::new(map_config, generation_config.clone(), mutation_config.clone()),
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