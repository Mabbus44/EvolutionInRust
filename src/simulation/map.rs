pub mod entity;

use rand::Rng;
use entity::Entity;
use entity::EntityType;
use entity::carnivore::Carnivore;
use entity::herbivore::Herbivore;
use entity::grass::Grass;
use crate::simulation::map::entity::{Action, Animal};
use std::{cmp, mem};

pub struct Map {
    pub animals: Vec<Vec<Entity>>,
    pub plants: Vec<Vec<Entity>>,
    pub size_x: usize,
    pub size_y: usize,
    pub carnivore_count: u32,
    pub herbivore_count: u32,
    pub grass_count: u32,
    pub neuron_count: usize,
    pub neuron_layer_count: usize,
    pub sense_radius: usize,
    input_count: usize,
}

impl Map {
    pub fn new(size_x: usize,
               size_y: usize,
               carnivore_count: u32,
               herbivore_count: u32,
               grass_count: u32,
               neuron_count: usize,
               neuron_layer_count: usize,
               sense_radius: usize
    ) -> Map {
        let mut map = Map {
            animals: Vec::new(),
            plants: Vec::new(),
            size_x,
            size_y,
            carnivore_count,
            herbivore_count,
            grass_count,
            neuron_count,
            neuron_layer_count,
            sense_radius,
            // Carnivore/Herbivore/Plant/Wall each has a input_side x input_side square of inputs, the centers are not included since it is occupied by the animal itself
            input_count: ((sense_radius * 2 + 1) * (sense_radius * 2 + 1) - 1) * 4,
        };
        map.generate_all_entities();
        map
    }
    fn generate_all_entities(&mut self) {
        let row: Vec<Entity> = vec![Entity::None; self.size_x];
        self.plants = vec![row; self.size_y];
        let row: Vec<Entity> = vec![Entity::None; self.size_x];
        self.animals = vec![row; self.size_y];
        self.generate_entities(self.carnivore_count, EntityType::Carnivore);
        self.generate_entities(self.herbivore_count, EntityType::Herbivore);
        self.generate_entities(self.grass_count, EntityType::Grass);
    }

    fn generate_entities(&mut self, entity_count: u32, entity_type: EntityType) {
        for _ in 0..entity_count {
            let mut retries: u8 = 0;
            while retries < 3 {
                let pos_x = rand::thread_rng().gen_range(0..self.size_x);
                let pos_y = rand::thread_rng().gen_range(0..self.size_y);
                let mut is_free = false;
                match entity_type {
                    EntityType::Herbivore | EntityType::Carnivore => {
                        is_free = matches!(self.animals[pos_y][pos_x], Entity::None);
                    }
                    EntityType::Grass => {
                        is_free = matches!(self.plants[pos_y][pos_x], Entity::None);
                    }
                    _ => {}
                }
                if is_free {
                    match entity_type {
                        EntityType::Grass => {
                            self.plants[pos_y][pos_x] = Entity::Grass(Grass {});
                        }
                        EntityType::Herbivore => {
                            self.animals[pos_y][pos_x] = Entity::Herbivore(Herbivore::new(self.input_count, self.neuron_count, self.neuron_layer_count));
                        }
                        EntityType::Carnivore => {
                            self.animals[pos_y][pos_x] = Entity::Carnivore(Carnivore::new(self.input_count, self.neuron_count, self.neuron_layer_count));
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

    pub fn tick(&mut self){
        self.calculate_actions();
        self.take_eat_actions();
        self.take_other_actions();
    }

    pub fn calculate_actions(&mut self) {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                if matches!(self.animals[y][x], Entity::Carnivore(_) | Entity::Herbivore(_)) {
                    let input = self.generate_input(x as i32, y as i32);
                    match &mut self.animals[y][x] {
                        Entity::Carnivore(carnivore) => {
                            carnivore.calculate_action(input);
                        }
                        Entity::Herbivore(herbivore) => {
                            herbivore.calculate_action(input);
                        }
                        _ => {unreachable!()}
                    }
                }
            }
        }
    }

    pub fn take_eat_actions(&mut self) {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                if let Entity::Carnivore(carnivore) = &self.animals[y][x] &&
                    matches!(carnivore.get_action_ref_immutable(), Action::Eat) &&
                    self.take_eat_action(x as i32, y as i32, EntityType::Herbivore) &&          // Do stuff in the middle of an If statement. A bit cursed but best I could think of.
                    let Entity::Carnivore(carnivore) = &mut self.animals[y][x] {
                        carnivore.take_eat_action();
                }
                if let Entity::Herbivore(herbivore) = &self.animals[y][x] &&
                    matches!(herbivore.get_action_ref_immutable(), Action::Eat) &&
                    self.take_eat_action(x as i32, y as i32, EntityType::Herbivore) &&          // Do stuff in the middle of an If statement. A bit cursed but best I could think of.
                    let Entity::Herbivore(herbivore) = &mut self.animals[y][x] {
                        herbivore.take_eat_action();
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
                if matches!(entity_type, EntityType::Herbivore) && matches!(self.animals[y][x], Entity::Herbivore(_)) {
                    self.animals[y][x] = Entity::None;
                    return true;
                }
                if matches!(entity_type, EntityType::Grass) && matches!(self.plants[y][x], Entity::Grass(_)) {
                    self.plants[y][x] = Entity::None;
                    return true;
                }
            }
        }
        false
    }

    pub fn take_other_actions(&mut self) {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                if let Entity::Carnivore(carnivore) = &self.animals[y][x] {
                    let action = carnivore.get_action_ref_immutable().clone();
                    if self.take_other_action(x, y, action) &&
                        let Entity::Carnivore(carnivore) = &mut self.animals[y][x] {
                        carnivore.take_other_action();
                    }
                }
                if let Entity::Herbivore(herbivore) = &self.animals[y][x] {
                    let action = herbivore.get_action_ref_immutable().clone();
                    if self.take_other_action(x, y, action) &&
                        let Entity::Herbivore(herbivore) = &mut self.animals[y][x] {
                        herbivore.take_other_action();
                    }
                }
            }
        }
    }

    pub fn take_other_action(&mut self, x: usize, y: usize, action: Action) -> bool {
        match action {
            Action::WalkDown => {
                if y > 0 && matches!(self.animals[y - 1][x], Entity::None) {
                    self.animals[y - 1][x] = mem::replace(&mut self.animals[y][x], Entity::None);
                    return true;
                }
            }
            Action::WalkUp => {
                if y + 1 < self.size_y && matches!(self.animals[y + 1][x], Entity::None) {
                    self.animals[y + 1][x] = mem::replace(&mut self.animals[y][x], Entity::None);
                    return true;
                }
            }
            Action::WalkLeft => {
                if x > 0 && matches!(self.animals[y][x - 1], Entity::None) {
                    self.animals[y][x - 1] = mem::replace(&mut self.animals[y][x], Entity::None);
                    return true;
                }
            }
            Action::WalkRight => {
                if x + 1 < self.size_x && matches!(self.animals[y][x + 1], Entity::None) {
                    self.animals[y][x + 1] = mem::replace(&mut self.animals[y][x], Entity::None);
                    return true;
                }
            }
            Action::None => {}
            Action::Eat => {}
        }
        return false;
    }

    pub fn generate_input(&self, x_center:i32, y_center:i32) -> Vec<f32> {
        let side = self.sense_radius * 2 + 1;
        let mut ret: Vec<f32> = Vec::with_capacity((side * side - 1) * 4 - 4);
        ret.append(&mut self.generate_partial_input(x_center, y_center, EntityType::None));
        ret.append(&mut self.generate_partial_input(x_center, y_center, EntityType::Carnivore));
        ret.append(&mut self.generate_partial_input(x_center, y_center, EntityType::Herbivore));
        ret.append(&mut self.generate_partial_input(x_center, y_center, EntityType::Grass));
        ret
    }

    pub fn generate_partial_input(&self, x_center:i32, y_center:i32, input_type: EntityType) -> Vec<f32>{
        let side = self.sense_radius * 2 + 1;
        let mut ret: Vec<f32> = Vec::with_capacity(side * side - 1);
        for y in y_center - self.sense_radius as i32..y_center + 1 + self.sense_radius as i32 {
            for x in x_center - self.sense_radius as i32..x_center + 1 + self.sense_radius as i32 {
                if x == x_center && y == y_center {
                    continue;
                }
                let mut input_val :f32 = 0.0;
                if x < 0 || y < 0 || x >= self.size_x as i32 || y >= self.size_y as i32 {
                    if matches!(input_type, EntityType::None) {
                        input_val = 1.0;
                    }
                }
                else {
                    match input_type {
                        EntityType::Herbivore => {
                            if matches!(self.animals[y as usize][x as usize], Entity::Herbivore(_)) {
                                input_val = 1.0;
                            }
                        }
                        EntityType::Carnivore => {
                            if matches!(self.animals[y as usize][x as usize], Entity::Carnivore(_)) {
                                input_val = 1.0;
                            }
                        }
                        EntityType::Grass => {
                            if matches!(self.plants[y as usize][x as usize], Entity::Grass(_)) {
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
                let mut entity: &Entity = &self.animals[y][x];
                if matches!(entity, Entity::None) {
                    entity = &self.plants[y][x];
                }
                match entity {
                    Entity::Carnivore(_) => {
                        row.push('X');
                    }
                    Entity::Herbivore(_) => {
                        row.push('o');
                    }
                    Entity::Grass(_) => {
                        row.push('.');
                    }
                    _ => {
                        row.push(' ');
                    }
                }
            }
            ret.push_str(&row);
            ret.push('\n');
        }
        ret
    }
}


