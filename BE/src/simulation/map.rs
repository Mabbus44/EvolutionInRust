pub mod entity;
mod sensing;

use rand::RngExt;
use entity::EntityType;
use entity::carnivore::Carnivore;
use entity::herbivore::Herbivore;
use entity::{Action, EntityRef, animal::Animal};
use sensing::*;
use std::{cmp, mem};
use super::record::GenerationRecording;
use super::config::*;

pub struct Map {
    animals: Vec<Vec<EntityRef>>,
    plants: Vec<Vec<EntityRef>>,
    carnivores: Vec<Option<Carnivore>>,
    herbivores: Vec<Option<Herbivore>>,
    best_carnivores: Vec<Carnivore>,
    best_herbivores: Vec<Herbivore>,
    carnivore_start_count: u32,
    herbivore_start_count: u32,
    grass_start_count: u32,
    carnivore_count: u32,
    herbivore_count: u32,
    grass_count: u32,
    neuron_count: usize,
    neuron_layer_count: usize,
    sense_radius: usize,
    carnivore_max_energy: u32,
    herbivore_max_energy: u32,
    input_count: usize,
    size_x: usize,
    size_y: usize,
    tick: usize,
    generation_over: bool,
    record: bool,
    recording: Option<GenerationRecording>,
    generation_config: GenerationConfig,
    mutation_config: MutationConfig
}

impl Map {
    pub fn new(
        map_config: MapConfig,
        generation_config: GenerationConfig,
        mutation_config: MutationConfig
    ) -> Map {
        let mut map = Map {
            animals: vec![vec![EntityRef::None; map_config.size_x]; map_config.size_y],
            plants: vec![vec![EntityRef::None; map_config.size_x]; map_config.size_y],
            carnivores: Vec::with_capacity(map_config.carnivore_count as usize),
            herbivores: Vec::with_capacity(map_config.herbivore_count as usize),
            best_carnivores: Vec::new(),
            best_herbivores: Vec::new(),
            carnivore_start_count: map_config.carnivore_count,
            herbivore_start_count: map_config.herbivore_count,
            grass_start_count: map_config.grass_count,
            carnivore_count: 0,
            herbivore_count: 0,
            grass_count: 0,
            neuron_count: map_config.neuron_count,
            neuron_layer_count: map_config.neuron_layer_count,
            sense_radius: map_config.sense_radius,
            carnivore_max_energy: map_config.carnivore_max_energy,
            herbivore_max_energy: map_config.herbivore_max_energy,
            size_x: map_config.size_x,
            size_y: map_config.size_y,
            generation_over: false,
            record: map_config.record,
            generation_config,
            mutation_config,
            // Carnivore/Herbivore/Plant/Wall each has an input_side x input_side square of inputs, the centers are not included since it is occupied by the animal itself
            input_count: ((map_config.sense_radius * 2 + 1) * (map_config.sense_radius * 2 + 1) - 1) * 4,
            tick: 0,
            recording: None,
        };
        map.generate_entities(map.carnivore_start_count, EntityType::Carnivore, false);
        map.generate_entities(map.herbivore_start_count, EntityType::Herbivore, false);
        map.generate_entities(map.grass_start_count, EntityType::Grass, false);
        if map.record {
            map.recording = Some(GenerationRecording::new(&map));
        }
        map
    }

    pub fn start_new_generation(&mut self) {
        if !self.generation_over {
            panic!("Tried to start a new generation before the old one was over");
        }
        self.tick = 0;
        self.carnivore_count = 0;
        self.herbivore_count = 0;
        self.grass_count = 0;
        self.generation_over = false;
        self.plants = vec![vec![EntityRef::None; self.size_x]; self.size_y];
        self.animals = vec![vec![EntityRef::None; self.size_x]; self.size_y];
        self.carnivores = Vec::with_capacity(self.carnivore_start_count as usize);
        self.herbivores = Vec::with_capacity(self.herbivore_start_count as usize);
        self.generate_entities(self.carnivore_start_count, EntityType::Carnivore, true);
        self.generate_entities(self.herbivore_start_count, EntityType::Herbivore, true);
        self.generate_entities(self.grass_start_count, EntityType::Grass, true);
        self.best_carnivores = Vec::new();
        self.best_herbivores = Vec::new();
        if self.record {
            self.recording = Some(GenerationRecording::new(self));
        }
    }

    pub fn get_carnivores(&self) -> &Vec::<Option<Carnivore>> {
        &self.carnivores
    }

    pub fn get_herbivores(&self) -> &Vec::<Option<Herbivore>> {
        &self.herbivores
    }

    pub fn get_grass_count(&self) -> u32 {
        self.grass_count
    }

    pub fn get_plants(&self) -> &Vec<Vec<EntityRef>> {
        &self.plants
    }

    fn move_remaining_animals_to_best(&mut self) {
        let mut id: u32 = 0;
        while self.herbivore_count > 0 && self.best_herbivores.len() < self.generation_config.best_herbivore_count as usize && id < self.herbivores.len() as u32 {
            match &self.herbivores[id as usize] {
                Some(herbivore) => {
                    self.best_herbivores.push(herbivore.clone());
                    self.herbivore_count -= 1;
                }
                None => {}
            }
            id += 1;
        }
        id = 0;
        while self.carnivore_count > 0 && self.best_carnivores.len() < self.generation_config.best_carnivore_count as usize && id < self.carnivores.len() as u32  {
            match &self.carnivores[id as usize] {
                Some(carnivore) => {
                    self.best_carnivores.push(carnivore.clone());
                    self.carnivore_count -= 1;
                }
                None => {}
            }
            id += 1;
        }
        if self.best_carnivores.len() == 0 || self.best_herbivores.len() == 0 {
            panic!("Best carnivores or herbivores is empty");
        }
    }

    pub fn handle_and_return_generation_over(&mut self) -> bool {
        let old_val = self.generation_over;
        if self.generation_config.max_ticks_per_generation >= 0 && self.tick >= self.generation_config.max_ticks_per_generation as usize {
            self.generation_over = true;
        }
        if self.generation_config.all_entities_must_be_under_min_levels {
            if (self.generation_config.carnivore_count >= 0 && self.carnivore_count as i32 <= self.generation_config.carnivore_count) &&
                (self.generation_config.herbivore_count >= 0 && self.herbivore_count as i32 <= self.generation_config.herbivore_count) &&
                (self.generation_config.grass_count >= 0 && self.grass_count as i32 <= self.generation_config.grass_count) {
                self.generation_over = true;
            }
        } else {
            if (self.generation_config.carnivore_count >= 0 && self.carnivore_count as i32 <= self.generation_config.carnivore_count) ||
                (self.generation_config.herbivore_count >= 0 && self.herbivore_count as i32 <= self.generation_config.herbivore_count) ||
                (self.generation_config.grass_count >= 0 && self.grass_count as i32 <= self.generation_config.grass_count) {
                self.generation_over = true;
            }
        }
        if !old_val && self.generation_over {
            self.move_remaining_animals_to_best();
        }
        self.generation_over
    }

    fn generate_entities(&mut self, entity_count: u32, entity_type: EntityType, from_best: bool) {
        for _ in 0..entity_count {
            let mut retries: u8 = 0;
            while retries < 3 {
                let pos_x = rand::rng().random_range(0..self.size_x);
                let pos_y = rand::rng().random_range(0..self.size_y);
                let mut is_free = false;
                match entity_type {
                    EntityType::Herbivore | EntityType::Carnivore => {
                        is_free = matches!(self.animals[pos_y][pos_x], EntityRef::None);
                    }
                    EntityType::Grass => {
                        is_free = matches!(self.plants[pos_y][pos_x], EntityRef::None);
                    }
                    _ => {}
                }
                if is_free {
                    match entity_type {
                        EntityType::Grass => {
                            self.plants[pos_y][pos_x] = EntityRef::Grass;
                            self.grass_count += 1;
                        }
                        EntityType::Herbivore => {
                            self.animals[pos_y][pos_x] = EntityRef::Herbivore(self.herbivores.len());
                            if from_best {
                                let best_id = rand::rng().random_range(0..self.best_herbivores.len());
                                self.herbivores.push(Some(Herbivore::new_from_parent(&self.mutation_config, &self.best_herbivores[best_id], self.herbivore_max_energy, pos_x, pos_y)));
                            } else {
                                self.herbivores.push(Some(Herbivore::new(self.input_count, self.neuron_count, self.neuron_layer_count, self.herbivore_max_energy, pos_x, pos_y)));
                            }
                            self.herbivore_count += 1;
                        }
                        EntityType::Carnivore => {
                            self.animals[pos_y][pos_x] = EntityRef::Carnivore(self.carnivores.len());
                            if from_best {
                                let best_id = rand::rng().random_range(0..self.best_carnivores.len());
                                self.carnivores.push(Some(Carnivore::new_from_parent(&self.mutation_config, &self.best_carnivores[best_id], self.carnivore_max_energy, pos_x, pos_y)));
                            } else {
                                self.carnivores.push(Some(Carnivore::new(self.input_count, self.neuron_count, self.neuron_layer_count, self.carnivore_max_energy, pos_x, pos_y)));
                            }
                            self.carnivore_count += 1;
                        }
                        _ => {}
                    }
                    break;
                }
                else {
                    retries += 1;
                }
            }
        }
    }

    pub fn tick(&mut self) {
        self.calculate_actions();
        if self.record {
            self.recording.as_mut().unwrap().record_tick(&self.carnivores, &self.herbivores);
        }
        self.take_eat_actions();
        self.take_other_actions();
        self.tick += 1;
    }

    #[allow(dead_code)]
    pub fn get_tick_count(&self) -> usize { self.tick }

    pub fn calculate_actions(&mut self) {
        for i in 0..self.carnivores.len()  {
            if self.carnivores[i].is_none() {
                continue;
            }
            let (x, y) = self.carnivores[i].as_ref().unwrap().get_position();
            let input = generate_input(&self.animals, self.sense_radius, x as i32, y as i32);
            self.carnivores[i].as_mut().unwrap().calculate_action(input);
        }
        for i in 0..self.herbivores.len()  {
            if self.herbivores[i].is_none() {
                continue;
            }
            let (x, y) = self.herbivores[i].as_ref().unwrap().get_position();
            let input = generate_input(&self.animals, self.sense_radius, x as i32, y as i32);
            self.herbivores[i].as_mut().unwrap().calculate_action(input);
        }
    }

    pub fn take_eat_actions(&mut self) {
        for i in 0..self.carnivores.len() {
            if self.carnivores[i].is_none() {
                continue;
            }
            let (x, y) = self.carnivores[i].as_ref().unwrap().get_position();
            if matches!(self.carnivores[i].as_ref().unwrap().get_action_ref_immutable(), Action::Eat) {
                if self.take_eat_action(x as i32, y as i32, EntityType::Herbivore) {
                    self.carnivores[i].as_mut().unwrap().take_eat_action();
                } else {
                    self.carnivores[i].as_mut().unwrap().add_to_energy(-1);
                }
                if self.carnivores[i].as_ref().unwrap().get_energy() <= 0 {
                    self.kill_animal_by_id(i, EntityType::Carnivore);
                }
            }
        }
        for i in 0..self.herbivores.len() {
            if self.herbivores[i].is_none() {
                continue;
            }
            let (x, y) = self.herbivores[i].as_ref().unwrap().get_position();
            if matches!(self.herbivores[i].as_ref().unwrap().get_action_ref_immutable(), Action::Eat) {
                if self.take_eat_action(x as i32, y as i32, EntityType::Grass) {
                    self.herbivores[i].as_mut().unwrap().take_eat_action();
                } else {
                    self.herbivores[i].as_mut().unwrap().add_to_energy(-1);
                }
                if self.herbivores[i].as_ref().unwrap().get_energy() <= 0 {
                    self.kill_animal_by_id(i, EntityType::Herbivore);
                }
            }
        }
    }

    pub fn take_eat_action(&mut self, x_in: i32, y_in: i32, entity_type: EntityType) -> bool{
        let eat_range = 2;
        let x_min = cmp::max(x_in - eat_range, 0) as usize;
        let x_max = cmp::min(x_in + eat_range + 1, self.size_x as i32) as usize;
        let y_min = cmp::max(y_in - eat_range, 0) as usize;
        let y_max = cmp::min(y_in + eat_range + 1, self.size_y as i32) as usize;
        for y in y_min..y_max {
            for x in x_min..x_max {
                if x == x_in as usize && y == y_in as usize {
                    continue;
                }
                if matches!(entity_type, EntityType::Herbivore) && let EntityRef::Herbivore(_) = &self.animals[y][x] {
                    self.kill_animal_by_pos(x, y);
                    return true;
                }
                if matches!(entity_type, EntityType::Grass) && matches!(self.plants[y][x], EntityRef::Grass) {
                    if self.record && self.recording.is_some() {
                        self.recording.as_mut().unwrap().add_dead_grass(self.tick, x, y);
                    }
                    self.plants[y][x] = EntityRef::None;
                    self.grass_count -= 1;
                    return true;
                }
            }
        }
        false
    }

    pub fn take_other_actions(&mut self) {
        for i in 0..self.carnivores.len() {
            if self.carnivores[i].is_none() {
                continue;
            }
            let (x, y) = self.carnivores[i].as_ref().unwrap().get_position();
            let action = self.carnivores[i].as_ref().unwrap().get_action_ref_immutable().clone();
            if self.take_other_action(x, y, action) {
                self.carnivores[i].as_mut().unwrap().take_other_action();
            } else {
                self.carnivores[i].as_mut().unwrap().add_to_energy(-1);
            }
            if self.carnivores[i].as_ref().unwrap().get_energy() <= 0 {
                self.kill_animal_by_id(i, EntityType::Carnivore);
            }
        }
        for i in 0..self.herbivores.len() {
            if self.herbivores[i].is_none() {
                continue;
            }
            let (x, y) = self.herbivores[i].as_ref().unwrap().get_position();
            let action = self.herbivores[i].as_ref().unwrap().get_action_ref_immutable().clone();
            if self.take_other_action(x, y, action) {
                self.herbivores[i].as_mut().unwrap().take_other_action();
            } else {
                self.herbivores[i].as_mut().unwrap().add_to_energy(-1);
            }
            if self.herbivores[i].as_ref().unwrap().get_energy() <= 0 {
                self.kill_animal_by_id(i, EntityType::Herbivore);
            }
        }
    }

    pub fn take_other_action(&mut self, x: usize, y: usize, action: Action) -> bool {
        match action {
            Action::WalkDown => {
                if y == 0 {
                    return false;
                }
                return self.move_animal(x, y, x, y - 1);
            }
            Action::WalkUp => {
                return self.move_animal(x, y, x, y + 1);
            }
            Action::WalkLeft => {
                if x == 0 {
                    return false;
                }
                return self.move_animal(x, y, x - 1, y);
            }
            Action::WalkRight => {
                return self.move_animal(x, y, x + 1, y);
            }
            Action::None => {}
            Action::Eat => {}
        }
        false
    }

    pub fn move_animal(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        if to_x >= self.size_x || to_y >= self.size_y ||
            !matches!(self.animals[to_y][to_x], EntityRef::None) || matches!(self.animals[from_y][from_x], EntityRef::None) {
            return false;
        }
        if let EntityRef::Carnivore(id) = self.animals[from_y][from_x] {
            self.carnivores[id].as_mut().unwrap().set_position(to_x, to_y);
        }
        if let EntityRef::Herbivore(id) = self.animals[from_y][from_x] {
            self.herbivores[id].as_mut().unwrap().set_position(to_x, to_y);
        }
        self.animals[to_y][to_x] = mem::replace(&mut self.animals[from_y][from_x], EntityRef::None);
        true
    }

    pub fn kill_animal_by_pos(&mut self, x: usize, y: usize) {
        match self.animals[y][x] {
            EntityRef::Herbivore(id) => {
                if self.herbivore_count <= self.generation_config.best_herbivore_count {
                    self.best_herbivores.push(self.herbivores[id].as_ref().unwrap().clone());
                }
                self.animals[y][x] = EntityRef::None;
                self.herbivores[id] = None;
                self.herbivore_count -= 1;
            }
            EntityRef::Carnivore(id) => {
                if self.carnivore_count <= self.generation_config.best_carnivore_count {
                    self.best_carnivores.push(self.carnivores[id].as_ref().unwrap().clone());
                }
                self.animals[y][x] = EntityRef::None;
                self.carnivores[id] = None;
                self.carnivore_count -= 1;
            }
            EntityRef::Grass => {panic!("Tried to kill grass at {}, {}", x, y); }
            EntityRef::None => {panic!("Tried to kill none at {}, {}", x, y);}
        }
    }

    pub fn kill_animal_by_id(&mut self, id: usize, entity_type: EntityType) {
        match entity_type {
            EntityType::Herbivore => {
                let (x, y) = self.herbivores[id].as_ref().unwrap().get_position();
                self.kill_animal_by_pos(x, y);
            }
            EntityType::Carnivore => {
                let (x, y) = self.carnivores[id].as_ref().unwrap().get_position();
                self.kill_animal_by_pos(x, y);
            }
            EntityType::Grass => {panic!("Invalid EntityType Grass in kill_animal_by_id")}
            EntityType::None => {panic!("Invalid EntityType None in kill_animal_by_id")}
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret : String = "".to_string();
        for y in 0..self.size_y {
            let mut row : String = "".to_string();
            for x in 0..self.size_x {
                match &self.animals[y][x] {
                    EntityRef::Carnivore(_) => {
                        row.push('X');
                    }
                    EntityRef::Herbivore(_) => {
                        row.push('o');
                    }
                    EntityRef::Grass => {}
                    EntityRef::None => {
                        row.push(' ');
                    }
                }
                if matches!(&self.plants[y][x], EntityRef::Grass) {
                    row.push('.');
                }
            }
            ret.push_str(&row);
            ret.push('\n');
        }
        ret
    }
    
    pub fn get_recording(&self) -> Option<GenerationRecording> {
        self.recording.clone()
    }
}


