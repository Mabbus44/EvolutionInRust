use super::entity::{EntityType, EntityRef};

pub fn generate_input(entities: &Vec<Vec<EntityRef>>, sense_radius: usize, x_center:i32, y_center:i32) -> Vec<f64> {
    let side = sense_radius * 2 + 1;
    let mut ret: Vec<f64> = Vec::with_capacity((side * side - 1) * 4 - 4);
    ret.append(&mut generate_partial_input(entities, sense_radius, x_center, y_center, EntityType::None));
    ret.append(&mut generate_partial_input(entities, sense_radius, x_center, y_center, EntityType::Carnivore));
    ret.append(&mut generate_partial_input(entities, sense_radius, x_center, y_center, EntityType::Herbivore));
    ret.append(&mut generate_partial_input(entities, sense_radius, x_center, y_center, EntityType::Grass));
    ret
}

pub fn generate_partial_input(entities: &Vec<Vec<EntityRef>>, sense_radius: usize, x_center:i32, y_center:i32, input_type: EntityType) -> Vec<f64>{
    let side = sense_radius * 2 + 1;
    let mut ret: Vec<f64> = Vec::with_capacity(side * side - 1);
    for y in y_center - sense_radius as i32..y_center + 1 + sense_radius as i32 {
        for x in x_center - sense_radius as i32..x_center + 1 + sense_radius as i32 {
            if x == x_center && y == y_center {
                continue;
            }
            let mut input_val :f64 = 0.0;
            if x < 0 || y < 0 || y >= entities.len() as i32 || x >= entities[0].len() as i32 {
                if matches!(input_type, EntityType::None) {
                    input_val = 1.0;
                }
            }
            else {
                match input_type {
                    EntityType::Herbivore => {
                        if matches!(entities[y as usize][x as usize], EntityRef::Herbivore(_)) {
                            input_val = 1.0;
                        }
                    }
                    EntityType::Carnivore => {
                        if matches!(entities[y as usize][x as usize], EntityRef::Carnivore(_)) {
                            input_val = 1.0;
                        }
                    }
                    EntityType::Grass => {
                        if matches!(entities[y as usize][x as usize], EntityRef::Grass) {
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