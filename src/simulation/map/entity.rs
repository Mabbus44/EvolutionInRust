pub mod grass;
pub mod herbivore;
pub mod carnivore;
mod neuron;

use rand::Rng;
use carnivore::Carnivore;
use grass::Grass;
use herbivore::Herbivore;
use crate::simulation::map::entity::neuron::Neuron;

#[derive(Clone)]
pub enum Entity {
    Grass(Grass),
    Herbivore(Herbivore),
    Carnivore(Carnivore),
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
const ACTION_COUNT: usize = 6;

pub trait Animal {
    fn get_neurons_ref_mut(&mut self) -> &mut Vec<Vec<Neuron>>;
    fn get_action_ref_mut(&mut self) -> &mut Action;
    fn get_action_ref_immutable(&self) -> &Action;
    fn take_other_action(&mut self);
    fn take_eat_action(&mut self);
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
            let mut constants: Vec<f32> = Vec::with_capacity(input_count);
            for _ in 0..input_count {
                constants.push(rand::thread_rng().gen_range(-1.0..1.0));
            }
            ret.push(Neuron::new (constants));
        }
        ret
    }

    fn calculate_action(&mut self, mut inputs: Vec<f32>)  {
        for neron_row in self.get_neurons_ref_mut().iter_mut() {
            let mut outputs: Vec<f32> = Vec::with_capacity(neron_row.len());
            for neuron in neron_row.iter_mut() {
            outputs.push(neuron.calculate(&inputs));
            }
            inputs = outputs;
        }
        if inputs.len() != ACTION_COUNT {
            panic!("Used {} possible action instead of {}", inputs.len(), ACTION_COUNT);
        }
        let mut max: f32 = inputs[0];
        let action = self.get_action_ref_mut();
        *action = Action::WalkLeft;
        Self::compare_actions(&mut max, inputs[0], action, Action::WalkLeft);
        Self::compare_actions(&mut max, inputs[1], action, Action::WalkRight);
        Self::compare_actions(&mut max, inputs[2], action, Action::WalkUp);
        Self::compare_actions(&mut max, inputs[3], action, Action::WalkDown);
        Self::compare_actions(&mut max, inputs[4], action, Action::Eat);
        Self::compare_actions(&mut max, inputs[5], action, Action::None);
    }

    fn compare_actions(max: &mut f32, compare: f32, action: &mut Action, new_action: Action) {
        if *max < compare {
            *max = compare;
            *action = new_action;
        }
    }
}