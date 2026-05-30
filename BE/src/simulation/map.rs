pub mod entity;

use rand::RngExt;
use entity::EntityType;
use entity::carnivore::Carnivore;
use entity::herbivore::Herbivore;
use entity::{Action, EntityRef, animal::Animal};
use std::{cmp, mem};
use super::record::{AnimalRecord, GenerationRecording};
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
    generation: usize,
    tick: usize,
    generation_over: bool,
    record: bool,
    current_generation_record: Option<GenerationRecording>,
    complete_record: Vec<GenerationRecording>,
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
            animals: Vec::new(),
            plants: Vec::new(),
            carnivores: Vec::new(),
            herbivores: Vec::new(),
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
            // Carnivore/Herbivore/Plant/Wall each has a input_side x input_side square of inputs, the centers are not included since it is occupied by the animal itself
            input_count: ((map_config.sense_radius * 2 + 1) * (map_config.sense_radius * 2 + 1) - 1) * 4,
            generation: 0,
            tick: 0,
            current_generation_record: None,
            complete_record: Vec::new(),
        };
        map.reset();
        map
    }
    fn reset(&mut self) {
        self.tick = 0;
        self.generation = 0;
        self.carnivore_count = 0;
        self.herbivore_count = 0;
        self.grass_count = 0;
        self.generation_over = false;
        let row: Vec<EntityRef> = vec![EntityRef::None; self.size_x];
        self.plants = vec![row; self.size_y];
        let row: Vec<EntityRef> = vec![EntityRef::None; self.size_x];
        self.animals = vec![row; self.size_y];
        self.carnivores = Vec::with_capacity(self.carnivore_start_count as usize);
        self.herbivores = Vec::with_capacity(self.herbivore_start_count as usize);
        self.best_carnivores = Vec::new();
        self.best_herbivores = Vec::new();
        self.generate_entities(self.carnivore_start_count, EntityType::Carnivore, false);
        self.generate_entities(self.herbivore_start_count, EntityType::Herbivore, false);
        self.generate_entities(self.grass_start_count, EntityType::Grass, false);
        if self.record {
            self.complete_record.clear();
            self.init_current_generation_recording();
        }
    }

    pub fn start_new_generation(&mut self) {
        if let Some(record) = self.current_generation_record.take() {
            self.complete_record.push(record);
        }
        self.move_remaining_animals_to_best();
        if !self.generation_over {
            // When generation_over is set generation is increased, if it is not set generation have to be increased here
            self.generation += 1;
        }
        self.tick = 0;
        self.carnivore_count = 0;
        self.herbivore_count = 0;
        self.grass_count = 0;
        self.generation_over = false;
        let row: Vec<EntityRef> = vec![EntityRef::None; self.size_x];
        self.plants = vec![row; self.size_y];
        let row: Vec<EntityRef> = vec![EntityRef::None; self.size_x];
        self.animals = vec![row; self.size_y];
        self.carnivores = Vec::with_capacity(self.carnivore_start_count as usize);
        self.herbivores = Vec::with_capacity(self.herbivore_start_count as usize);
        self.generate_entities(self.carnivore_start_count, EntityType::Carnivore, true);
        self.generate_entities(self.herbivore_start_count, EntityType::Herbivore, true);
        self.generate_entities(self.grass_start_count, EntityType::Grass, true);
        self.best_carnivores = Vec::new();
        self.best_herbivores = Vec::new();
        if self.record {
            self.init_current_generation_recording();
        }
    }

    fn init_current_generation_recording(&mut self){
        self.current_generation_record = Some(
            GenerationRecording::new(
                self.carnivores.len(),
                self.herbivores.len(),
                self.grass_count as usize,
                self.carnivores.clone(),
                self.herbivores.clone()
            )
        );
        let mut y = 0;
        for row in &self.plants {
            let mut x = 0;
            for plant in row {
                match plant {
                    EntityRef::Grass => {
                        self.current_generation_record.as_mut().unwrap().grass_at_start.push((x, y));
                    }
                    _ => {}
                }
                x += 1;
            }
            y += 1;
        }
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

    pub fn set_and_return_generation_over(&mut self) -> bool {
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
            self.generation += 1;
        }
        self.generation_over
    }

    pub fn is_simulation_over(&self) -> bool {
        self.generation >= self.generation_config.max_generation_count as usize
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
            self.record_tick();
        }
        self.take_eat_actions();
        self.take_other_actions();
        self.tick += 1;
    }

    pub fn record_tick(&mut self) {
        let mut temp_animal_record: Option<AnimalRecord> = None;
        for id in 0..self.carnivores.len() {
            match &self.carnivores[id] {
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
                self.current_generation_record.as_mut().unwrap().carnivore_records[id].push(record);
            }
        }
        for id in 0..self.herbivores.len() {
            match &self.herbivores[id] {
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
                self.current_generation_record.as_mut().unwrap().herbivore_records[id].push(record);
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_tick_count(&self) -> usize { self.tick }

    pub fn calculate_actions(&mut self) {
        for i in 0..self.carnivores.len()  {
            if self.carnivores[i].is_none() {
                continue;
            }
            let (x, y) = self.carnivores[i].as_ref().unwrap().get_position();
            let input = self.generate_input(x as i32, y as i32);
            self.carnivores[i].as_mut().unwrap().calculate_action(input);
        }
        for i in 0..self.herbivores.len()  {
            if self.herbivores[i].is_none() {
                continue;
            }
            let (x, y) = self.herbivores[i].as_ref().unwrap().get_position();
            let input = self.generate_input(x as i32, y as i32);
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
                    if self.record && self.current_generation_record.is_some() {
                        self.current_generation_record.as_mut().unwrap().dead_grass.push((self.tick, x, y));
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

    pub fn generate_input(&self, x_center:i32, y_center:i32) -> Vec<f64> {
        let side = self.sense_radius * 2 + 1;
        let mut ret: Vec<f64> = Vec::with_capacity((side * side - 1) * 4 - 4);
        ret.append(&mut self.generate_partial_input(x_center, y_center, EntityType::None));
        ret.append(&mut self.generate_partial_input(x_center, y_center, EntityType::Carnivore));
        ret.append(&mut self.generate_partial_input(x_center, y_center, EntityType::Herbivore));
        ret.append(&mut self.generate_partial_input(x_center, y_center, EntityType::Grass));
        ret
    }

    pub fn generate_partial_input(&self, x_center:i32, y_center:i32, input_type: EntityType) -> Vec<f64>{
        let side = self.sense_radius * 2 + 1;
        let mut ret: Vec<f64> = Vec::with_capacity(side * side - 1);
        for y in y_center - self.sense_radius as i32..y_center + 1 + self.sense_radius as i32 {
            for x in x_center - self.sense_radius as i32..x_center + 1 + self.sense_radius as i32 {
                if x == x_center && y == y_center {
                    continue;
                }
                let mut input_val :f64 = 0.0;
                if x < 0 || y < 0 || x >= self.size_x as i32 || y >= self.size_y as i32 {
                    if matches!(input_type, EntityType::None) {
                        input_val = 1.0;
                    }
                }
                else {
                    match input_type {
                        EntityType::Herbivore => {
                            if matches!(self.animals[y as usize][x as usize], EntityRef::Herbivore(_)) {
                                input_val = 1.0;
                            }
                        }
                        EntityType::Carnivore => {
                            if matches!(self.animals[y as usize][x as usize], EntityRef::Carnivore(_)) {
                                input_val = 1.0;
                            }
                        }
                        EntityType::Grass => {
                            if matches!(self.plants[y as usize][x as usize], EntityRef::Grass) {
                                input_val = 1.0;
                            }
                        }
                        EntityType::None => {}
                    }
                }
                ret.push(input_val);
            }
        }
        ret
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
    
    pub fn get_recording(&self) -> Vec<GenerationRecording> {
        self.complete_record.clone()
    }
}


