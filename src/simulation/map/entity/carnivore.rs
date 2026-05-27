use super::{Action, animal::Animal, neuron::Neuron };
use crate::simulation::mutation::MutationConfig;

#[derive(Clone)]
pub struct Carnivore {
    neurons: Vec<Vec<Neuron>>,
    energy: i32,
    action: Action,
    pos_x: usize,
    pos_y: usize,
}

impl Carnivore {
    pub fn new(input_count: usize, neuron_count: usize, neuron_layer_count: usize, start_energy: u32, pos_x: usize, pos_y: usize) -> Carnivore {
        let neurons = Carnivore::generate_neuron_layers(input_count, neuron_count, neuron_layer_count);
        Carnivore { neurons, energy: start_energy as i32, action: Action::None, pos_x, pos_y }
    }

    pub fn new_from_parent(mutation_config: &MutationConfig, parent: &Carnivore, start_energy: u32, pos_x: usize, pos_y: usize) -> Carnivore {
        let mut ret: Carnivore = parent.clone();
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

impl Animal for Carnivore {
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
