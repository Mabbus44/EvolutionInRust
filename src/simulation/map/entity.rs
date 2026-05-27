pub mod herbivore;
pub mod carnivore;
pub mod animal;
mod neuron;

#[derive(Clone)]
pub enum EntityRef {
    Herbivore(usize),
    Carnivore(usize),
    Grass,
    None
}

pub enum EntityType {
    Grass,
    Herbivore,
    Carnivore,
    None
}

#[derive(Clone)]
pub enum Action {
    WalkLeft,
    WalkRight,
    WalkUp,
    WalkDown,
    Eat,
    None
}

impl Action {
    pub fn to_int(&self) -> usize {
        match self {
            Action::WalkLeft => 0,
            Action::WalkRight => 1,
            Action::WalkUp => 2,
            Action::WalkDown => 3,
            Action::Eat => 4,
            Action::None => 5,
        }
    }
}
const ACTION_COUNT: usize = 6;
