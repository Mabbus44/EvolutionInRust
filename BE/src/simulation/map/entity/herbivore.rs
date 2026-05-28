use serde::Serialize;
use crate::simulation::mutation::MutationConfig;
use super::neuron::Neuron;
use super::{Action, animal::Animal};

#[derive(Clone)]
#[derive(Serialize)]
pub struct Herbivore {
    neurons: Vec<Vec<Neuron>>,
    energy: i32,
    action: Action,
    pos_x: usize,
    pos_y: usize,
}

impl Herbivore {
    pub fn new(input_count: usize, neuron_count: usize, neuron_layer_count: usize, start_energy: u32, pos_x: usize, pos_y: usize) -> Herbivore {
        let neurons = Herbivore::generate_neuron_layers(input_count, neuron_count, neuron_layer_count);
        Herbivore { neurons, energy: start_energy as i32, action: Action::None, pos_x, pos_y }
    }

    pub fn new_from_parent(mutation_config: &MutationConfig, parent: &Herbivore, start_energy: u32, pos_x: usize, pos_y: usize) -> Herbivore {
        let mut ret: Herbivore = parent.clone();
        for neron_row in &mut ret.neurons {
            for neuron in neron_row {
                neuron.mutate(mutation_config);
            }
        }
        ret.set_position(pos_x, pos_y);
        ret.set_energy(start_energy as i32);
        ret
    }
}

impl Animal for Herbivore {
    fn get_neurons_ref_mut(&mut self) -> &mut Vec<Vec<Neuron>> {
        &mut self.neurons
    }
    fn get_neurons_ref_immutable(& self) -> &Vec<Vec<Neuron>> { &self.neurons }
    fn get_action_ref_mut(&mut self) -> &mut Action {
        &mut self.action
    }
    fn get_action_ref_immutable(&self) -> &Action { &self.action }
    fn get_energy(&self) -> i32 {
        self.energy
    }
    fn set_energy(&mut self, energy: i32){
        self.energy = energy;
    }
    fn add_to_energy(&mut self, energy: i32){
        self.energy += energy;
        if self.energy < 0 {
            self.energy = 0;
        }
    }
    fn set_position(&mut self, x: usize, y: usize){
        self.pos_x = x;
        self.pos_y = y;
    }
    fn get_position(&self) -> (usize, usize){
        (self.pos_x, self.pos_y)
    }
}