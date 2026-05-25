pub mod map;

use map::Map;

pub struct Simulation {
    map: Map
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            map: Map::new(
                20,
                10,
                2,
                4,
                10,
                20,
                2,
                3)
        }
    }

    pub fn simulate(&mut self){
        print!("{}", self.to_string());
        for _ in 0..10 {
            self.map.tick();
        }
        println!();
        println!();
        println!("********************************************************");
        println!();
        println!();
        print!("{}", self.to_string());
    }
    pub fn to_string(&self) -> String {
        self.map.to_string()
    }
}