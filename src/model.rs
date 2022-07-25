
extern crate csv;
use csv::ReaderBuilder;

use std::fs;

use super::*;

pub struct Model {
    name: String,
    layers: Vec<LayerDesc>,
    inputs: usize,
    outputs: usize,
    loss: Loss,
    opto: Opto,

    dataset: Vec<(Vector, Vector)>,
}

impl Model {
    pub fn new(name: &str) -> Self {
        Model {
            name: name.to_string(),
            layers: Vec::new(),
            inputs: 0,
            outputs: 0,
            loss: Loss::MAE,
            opto: Opto::Linear,
            dataset: Vec::new(),
        }
    }

    pub fn set_inputs(&mut self, size: usize) {
        self.inputs = size;
    }

    pub fn add_layer(&mut self, desc: LayerDesc, count: usize) {
        for i in 0..count {
            self.layers.push(desc);
        }
    }

    pub fn add_output(&mut self, desc: LayerDesc) {
        self.layers.push(desc);
    }

    pub fn set_loss(&mut self, loss: Loss) {
        self.loss = loss;
    }

    pub fn set_optomizer(&mut self, opto: Opto) {
        self.opto = opto;
    }

    pub fn load_csv<F: Fn(&Vec<String>, &mut Vec<f32>, &mut Vec<f32>)>(&mut self, filename: &str, predicate: F) {

        self.outputs = self.layers.last_mut().unwrap().size as usize;

        let contents = fs::read_to_string(
            self.name.to_string() + "/" + &filename + ".csv").unwrap();

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b',')
            .from_reader(contents.as_bytes());

        let mut features: Vec<String> = Vec::new();

        for result in reader.records() {
            let record = result.unwrap();
            features.clear();
    
            for f in &record {
                features.push(f.to_string());
            }   
    
            let mut inputs = Vec::new();
            let mut target = Vec::new();

            (predicate)(&features, &mut inputs, &mut target);

            if (inputs.len() != self.inputs) {
                panic!("{}", format!("Incorrect dataset target size! Must be: {} was: {}", self.inputs, inputs.len()));
            }

            if (target.len() != self.outputs) {
                panic!("{}", format!("Incorrect dataset input size! Must be: {} was: {}", self.outputs, target.len()));
            }

            let newin = Vector::from_fn(self.inputs as usize, |i, _| {
                inputs[i]
            });

            let newtar = Vector::from_fn(self.outputs as usize, |i, _| {
                target[i]
            });

            self.dataset.push((newin, newtar));
        }
    }
}

#[derive(Copy, Clone)]
pub struct LayerDesc {
    pub layr: Layr,
    pub actv: Actv,
    pub size: usize,
}