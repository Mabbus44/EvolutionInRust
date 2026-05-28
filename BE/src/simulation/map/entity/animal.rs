use rand::RngExt;
use super::{Action, ACTION_COUNT};
use super::neuron::Neuron;

pub trait Animal {
    fn get_neurons_ref_mut(&mut self) -> &mut Vec<Vec<Neuron>>;
    #[allow(dead_code)]
    fn get_neurons_ref_immutable(& self) -> &Vec<Vec<Neuron>>;
    fn get_action_ref_mut(&mut self) -> &mut Action;
    fn get_action_ref_immutable(&self) -> &Action;
    fn get_energy(&self) -> i32;
    fn set_energy(&mut self, energy: i32);
    fn add_to_energy(&mut self, energy: i32);
    fn set_position(&mut self, x: usize, y: usize);
    fn get_position(&self) -> (usize, usize);
    fn take_other_action(&mut self){
        match self.get_action_ref_immutable() {
            Action::WalkLeft | Action::WalkRight  | Action::WalkUp | Action::WalkDown => {
                self.add_to_energy(-3);
            }
            Action::Eat => {
                panic!("Called take_other_action for a animal that wanted to eat");
            }
            Action::None => {
                self.add_to_energy(-1);
            }
        }
    }
    fn take_eat_action(&mut self){
        self.add_to_energy(100);
    }
    fn generate_neuron_layers(input_count: usize, neuron_count: usize, neuron_layer_count: usize) -> Vec<Vec<Neuron>> {
        let mut neurons: Vec<Vec<Neuron>> = Vec::with_capacity(neuron_layer_count);
        for i in 0..neuron_layer_count {
            if i == 0 {
                neurons.push(Self::generate_neurons(input_count, neuron_count));
            }
            else {
                neurons.push(Self::generate_neurons(neuron_count, neuron_count));
            }
        }
        neurons.push(Self::generate_neurons(neuron_count, ACTION_COUNT));
        neurons
    }
    fn generate_neurons(input_count: usize, neuron_count: usize) -> Vec<Neuron> {
        let mut ret: Vec<Neuron> = Vec::with_capacity(neuron_count);
        for _ in 0..neuron_count {
            let mut constants: Vec<f64> = Vec::with_capacity(input_count);
            for _ in 0..input_count {
                constants.push(rand::rng().random_range(-1.0..1.0));
            }
            ret.push(Neuron::new (constants));
        }
        ret
    }

    fn calculate_action(&mut self, mut inputs: Vec<f64>)  {
        for neron_row in self.get_neurons_ref_mut().iter_mut() {
            let mut outputs: Vec<f64> = Vec::with_capacity(neron_row.len());
            for neuron in neron_row.iter_mut() {
                outputs.push(neuron.calculate(&inputs));
            }
            inputs = outputs;
        }
        if inputs.len() != ACTION_COUNT {
            panic!("Used {} possible action instead of {}", inputs.len(), ACTION_COUNT);
        }
        let mut max: f64 = inputs[0];
        let action = self.get_action_ref_mut();
        *action = Action::WalkLeft;
        Self::compare_actions(&mut max, inputs[0], action, Action::WalkLeft);
        Self::compare_actions(&mut max, inputs[1], action, Action::WalkRight);
        Self::compare_actions(&mut max, inputs[2], action, Action::WalkUp);
        Self::compare_actions(&mut max, inputs[3], action, Action::WalkDown);
        Self::compare_actions(&mut max, inputs[4], action, Action::Eat);
        Self::compare_actions(&mut max, inputs[5], action, Action::None);
    }

    fn compare_actions(max: &mut f64, compare: f64, action: &mut Action, new_action: Action) {
        if *max < compare {
            *max = compare;
            *action = new_action;
        }
    }
}