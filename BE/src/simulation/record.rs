use crate::simulation::map::entity::animal::Animal;
use super::map::entity::{Action};
use super::map::entity::carnivore::Carnivore;
use super::map::entity::herbivore::Herbivore;

#[derive(Clone)]
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
    pub fn to_json(&self) -> String {
        let mut ret: String = "{\"carnivores_at_start\":[".to_string();
        let mut first = true;
        for option in &self.carnivores_at_start {
            if !first {
                ret.push_str(",");
            }
            first = false;
            match option {
                Some(carnivore) => {
                    ret.push_str(&carnivore.to_json());
                }
                None => {
                    ret.push_str("{}");
                }
            }
        }
        ret.push_str("],\"herbivores_at_start\":[");
        first = true;
        for option in &self.herbivores_at_start {
            if !first {
                ret.push_str(",");
            }
            first = false;
            match option {
                Some(herbivore) => {
                    ret.push_str(&herbivore.to_json());
                }
                None => {
                    ret.push_str("{}");
                }
            }
        }
        ret.push_str("],\"grass_at_start\":[");
        first = true;
        for grass in &self.grass_at_start {
            if !first {
                ret.push_str(",");
            }
            first = false;
            ret.push_str(&format!("[{},{}]", grass.0, grass.1));
        }
        ret.push_str("],\"carnivore_records\":[");
        let mut first_row = true;
        for row in &self.carnivore_records {
            if !first_row {
                ret.push_str(",");
            }
            first_row = false;
            let mut first_record = true;
            ret.push_str("[");
            for record in row {
                if !first_record {
                    ret.push_str(",");
                }
                first_record = false;
                ret.push_str(&record.to_json());
            }
            ret.push_str("]");
        }
        ret.push_str("],\"herbivore_records\":[");
        first_row = true;
        for row in &self.herbivore_records {
            if !first_row {
                ret.push_str(",");
            }
            first_row = false;
            let mut first_record = true;
            ret.push_str("[");
            for record in row {
                if !first_record {
                    ret.push_str(",");
                }
                first_record = false;
                ret.push_str(&record.to_json());
            }
            ret.push_str("]");
        }
        ret.push_str("],\"dead_grass\":[");
        first = true;
        for grass in &self.dead_grass {
            if !first {
                ret.push_str(",");
            }
            first = false;
            ret.push_str(&format!("[{},{},{}]", grass.0, grass.1, grass.2));
        }
        ret.push_str("]}");
        ret
    }
}

#[derive(Clone)]
pub struct AnimalRecord {
    pub energy: i32,
    pub action: Action,
    pub pos_x: usize,
    pub pos_y: usize,
}

impl AnimalRecord {
    pub fn to_json(&self) -> String {
        format!("[{},{},{},{}]", self.energy, self.action.to_int(), self.pos_x, self.pos_y).to_string()
    }
}