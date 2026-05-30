use rand::RngExt;
use serde::Serialize;
use crate::json_helper::{serialize_f64_2dp, serialize_vec_f64_2dp};
use crate::simulation::config::MutationConfig;

#[derive(Clone)]
#[derive(Serialize)]
pub struct Neuron {
    #[serde(serialize_with = "serialize_vec_f64_2dp")]
    constants: Vec<f64>,
    #[serde(serialize_with = "serialize_f64_2dp")]
    output: f64
}

impl Neuron {
    pub fn new(constants: Vec<f64>) -> Neuron {
        Neuron { constants, output: 0.0 }
    }

    pub fn calculate(&mut self, inputs: &Vec<f64>) -> f64 {
        if inputs.len() != self.constants.len() {
            panic!("Tried to feed a {} len neuron a {} len input", self.constants.len(), inputs.len());
        }
        self.output = 0.0;
        for i in 0..self.constants.len() {
            self.output += inputs[i] * self.constants[i];
        }
        self.output
    }

    pub fn mutate(&mut self, mutation_config: &MutationConfig) {
        for constant in &mut self.constants {
            if rand::rng().random_bool(mutation_config.mutation_chance) {
                let mut min: f64 = *constant - mutation_config.max_mutation_amount;
                let mut max: f64 = *constant + mutation_config.max_mutation_amount;
                if min < -1.0 {
                    min = -1.0;
                }
                if max > 1.0 {
                    max = 1.0;
                }
                *constant = rand::rng().random_range(min..max)
            }
        }
    }
}