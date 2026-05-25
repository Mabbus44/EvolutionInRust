use crate::simulation::map::entity::{Action, Animal};
use crate::simulation::map::entity::neuron::Neuron;

#[derive(Clone)]
pub struct Herbivore {
    neurons: Vec<Vec<Neuron>>,
    action: Action
}

impl Herbivore {
    pub fn new(input_count: usize, neuron_count: usize, neuron_layer_count: usize) -> Herbivore {
        let neurons = Herbivore::generate_neuron_layers(input_count, neuron_count, neuron_layer_count);
        Herbivore { neurons, action: Action::None }
    }
}

impl Animal for Herbivore {
    fn get_neurons_ref_mut(&mut self) -> &mut Vec<Vec<Neuron>> {
        &mut self.neurons
    }
    fn get_action_ref_mut(&mut self) -> &mut Action {
        &mut self.action
    }
    fn get_action_ref_immutable(&self) -> &Action { &self.action }
    fn take_other_action(&mut self){
        self.get_action_ref_immutable();
        self.action = Action::None;
    }
    fn take_eat_action(&mut self){
        self.get_action_ref_immutable();
        self.action = Action::None;
    }
}