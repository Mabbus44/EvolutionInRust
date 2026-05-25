#[derive(Clone)]
pub struct Neuron {
    constants: Vec<f32>,
    output: f32
}

impl Neuron {
    pub fn new(constants: Vec<f32>) -> Neuron {
        Neuron { constants, output: 0.0 }
    }

    pub fn calculate(&mut self, inputs: &Vec<f32>) -> f32 {
        if inputs.len() != self.constants.len() {
            panic!("Tried to feed a {} len neuron a {} len input", self.constants.len(), inputs.len());
        }
        self.output = 0.0;
        for i in 0..self.constants.len() {
            self.output += inputs[i] * self.constants[i];
        }
        self.output
    }
}