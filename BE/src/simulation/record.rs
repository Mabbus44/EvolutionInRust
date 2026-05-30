use serde::Serialize;
use super::map::entity::{Action, EntityRef, animal::Animal};
use super::map::entity::carnivore::Carnivore;
use super::map::entity::herbivore::Herbivore;
use super::map::Map;

#[derive(Clone)]
#[derive(Serialize)]
pub struct GenerationRecording {
    carnivores_at_start: Vec<Option<Carnivore>>,
    herbivores_at_start: Vec<Option<Herbivore>>,
    grass_at_start: Vec<(usize, usize)>,
    carnivore_records: Vec<Vec<AnimalRecord>>,
    herbivore_records: Vec<Vec<AnimalRecord>>,
    dead_grass: Vec<(usize, usize, usize)>, //Tick, x, y
}

impl GenerationRecording {
    pub fn new(map: &Map) -> GenerationRecording {
        let carnivores = map.get_carnivores();
        let herbivores = map.get_herbivores();
        let mut ret = GenerationRecording {
            carnivores_at_start: carnivores.clone(),
            herbivores_at_start: herbivores.clone(),
            grass_at_start: Vec::with_capacity(map.get_grass_count() as usize),
            carnivore_records: vec![Vec::new(); carnivores.len()],
            herbivore_records: vec![Vec::new(); herbivores.len()],
            dead_grass: Vec::with_capacity(map.get_grass_count() as usize),
        };
        let mut y = 0;
        for row in map.get_plants() {
            let mut x = 0;
            for plant in row {
                match plant {
                    EntityRef::Grass => {
                        ret.grass_at_start.push((x, y));
                    }
                    _ => {}
                }
                x += 1;
            }
            y += 1;
        }
        ret
    }

    pub fn record_tick(&mut self, carnivores: &Vec<Option<Carnivore>>, herbivores: &Vec<Option<Herbivore>>) {
        let mut temp_animal_record: Option<AnimalRecord> = None;
        for id in 0..carnivores.len() {
            match &carnivores[id] {
                Some(carnivore) => {
                    let pos = carnivore.get_position();
                    temp_animal_record = Some(AnimalRecord {
                        energy: carnivore.get_energy(),
                        action: carnivore.get_action_ref_immutable().clone(),
                        pos_x: pos.0,
                        pos_y: pos.1,
                    });
                }
                None => {}
            }
            if let Some(record) = temp_animal_record.take() {
                self.carnivore_records[id].push(record);
            }
        }
        for id in 0..herbivores.len() {
            match &herbivores[id] {
                Some(herbivore) => {
                    let pos = herbivore.get_position();
                    temp_animal_record = Some(AnimalRecord {
                        energy: herbivore.get_energy(),
                        action: herbivore.get_action_ref_immutable().clone(),
                        pos_x: pos.0,
                        pos_y: pos.1,
                    });
                }
                None => {}
            }
            if let Some(record) = temp_animal_record.take() {
                self.herbivore_records[id].push(record);
            }
        }
    }

    pub fn add_dead_grass(&mut self, tick: usize, x: usize, y: usize) {
        self.dead_grass.push((tick, x, y));
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