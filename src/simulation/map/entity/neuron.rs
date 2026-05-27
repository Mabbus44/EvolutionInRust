use rand::Rng;
use crate::simulation::mutation::MutationConfig;

#[derive(Clone)]
pub struct Neuron {
    constants: Vec<f64>,
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
            if rand::thread_rng().gen_bool(mutation_config.mutation_chance) {
                let mut min: f64 = *constant - mutation_config.max_mutation_amount;
                let mut max: f64 = *constant + mutation_config.max_mutation_amount;
                if min < -1.0 {
                    min = -1.0;
                }
                if max > -1.0 {
                    max = 1.0;
                }
                *constant = rand::thread_rng().gen_range(min..max)
            }
        }
    }

    pub fn to_json(&self) -> String {
        let mut ret: String = "{\"constants\":[".to_string();
        let mut first = true;
        for row in &self.constants {
            if !first {
                ret.push_str(",");
            }
            first = false;
            ret.push_str(&format!("{:.2}", row));
        }
        ret.push_str(&format!("],\"output\":{:.2}}}",self.output));
        ret
    }
}