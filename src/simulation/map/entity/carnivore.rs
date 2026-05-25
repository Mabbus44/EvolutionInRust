use super::neuron::Neuron;
use super::{Action, Animal};

#[derive(Clone)]
pub struct Carnivore {
    neurons: Vec<Vec<Neuron>>,
    action: Action
}

impl Carnivore {
    pub fn new(input_count: usize, neuron_count: usize, neuron_layer_count: usize) -> Carnivore {
        let neurons = Carnivore::generate_neuron_layers(input_count, neuron_count, neuron_layer_count);
        Carnivore { neurons, action: Action::None }
    }
}

impl Animal for Carnivore {
    fn get_neurons_ref_mut(&mut self) -> &mut Vec<Vec<Neuron>> {
        &mut self.neurons
    }
    fn get_action_ref_mut(&mut self) -> &mut Action {
        &mut self.action
    }
    fn get_action_ref_immutable(&self) -> &Action { &self.action }
    fn take_other_action(&mut self){
        self.get_action_ref_immutable();
    }
    fn take_eat_action(&mut self){
        self.get_action_ref_immutable();
    }
}