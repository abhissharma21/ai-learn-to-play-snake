use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::configs::{IS_SAVE_BEST_NET, LOAD_FILE_NAME, SAVE_FILE_NAME};

#[derive(Clone, Serialize, Deserialize)]
pub struct Net {
    n_inputs: usize,
    layers: Vec<Layer>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Layer {
    nodes: Vec<Node>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Node {
    weights: Vec<f64>,
    bias: f64,
}

impl Net {
    #[must_use]
    pub fn new(layer_sizes: &[usize]) -> Self {
        assert!(!layer_sizes.len() >=2 ,    "Need atleast 2 layers");
        assert!(layer_sizes.iter().all(|&size| size > 0), "Empty layers not allowed");

        let mut layers = Vec::new();
        let first_layer_size = *layer_sizes.first().unwrap();
        let mut prev_layer_size = first_layer_size;

        for &layer_size in &layer_sizes[1..] {
            layers.push(Layer::new(layer_size, prev_layer_size));
        }

        Self {
            layers,
            n_inputs: first_layer_size,
        }
    }

    #[must_use]
    pub fn merge(&self, other: &Net) -> Self {
        assert_eq!(self.layers.len(), other.layers.len());

        let mut merged_layers = Vec::new();
        for i in 0..self.layers.len() {
            let merged_layer = &self.layers[i].merge(&other.layers[i]);
            merged_layers.push(merged_layer.clone());
        }

        Net {
            layers: merged_layers,
            n_inputs: self.n_inputs,
        }
    }

    #[must_use]
    pub fn predict(&self, inputs: Vec<f64>) -> Vec<f64> {
        assert_eq!(inputs.len(), self.n_inputs, "Bad input size, expected {:?} but got {:?}", self.n_inputs, inputs.len());
        let output = inputs;

        self.layers
            .iter()
            .flat_map(|layer| layer.predict(&output))
            .collect()

    }

    pub fn mutate(&mut self, rate: f64, magnitude: f64) {
        self.layers
            .iter_mut()
            .for_each(|l| l.mutate(rate, magnitude));
    }

    pub fn save(&self) {
        if !IS_SAVE_BEST_NET {
            return;
        }

        let path = Path::new(SAVE_FILE_NAME);
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    create_dir_all(path.parent().unwrap()).unwrap();
                    File::create(path).unwrap()
                } else {
                    panic!("Unexpected error: {err}");
                }
            }
        };

        let json: String = serde_json::to_string(&self).unwrap();
        file.write_all(json.as_bytes())
            .expect("Failed to write to network file");
    }

    #[must_use]
    pub fn load() -> Self {
        let mut file = File::open(LOAD_FILE_NAME).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();
        serde_json::from_str(&buff).unwrap()
    }

    #[must_use]
    pub fn get_bias(&self, layer_idx: usize) -> Vec<f64> {
        let mut res = Vec::new();
        for node in &self.layers[layer_idx].nodes {
            res.push(node.bias);
        }

        res
    }
}

impl Layer {
    fn new(layer_size: usize, prev_layer_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut nodes = Vec::new();

        for _ in 0..layer_size {
            let mut weights: Vec<f64> = Vec::new();
            for _ in 0..prev_layer_size {
                let random_weight: f64 = rng.gen_range(-1.0..1.0);
                weights.push(random_weight);
            }
            let bias: f64 = rng.gen_range(-1.0..1.0);
            nodes.push(Node { weights, bias });
        }

        Self { nodes }
    }

    fn merge(&self, other: &Layer) -> Self {
        assert_eq!(self.nodes.len(), other.nodes.len());
        let mut rng = rand::thread_rng();
        let mut nodes: Vec<Node> = Vec::new();

        for (node1, node2) in self.nodes.iter().zip(other.nodes.iter()) {
            let mut merged_weights = Vec::new();
            for (&weight1, &weight2) in node1.weights.iter().zip(node2.weights.iter()) {
                let selected_weight = if rng.gen::<bool>() { weight1 } else { weight2 };
                merged_weights.push(selected_weight);
            }
            let merged_bias = if rng.gen::<bool>() {
                node1.bias
            } else {
                node2.bias
            };
            nodes.push(Node {
                weights: merged_weights,
                bias: merged_bias,
            });
        }

        Self { nodes }
    }

    fn predict(&self, inputs: &[f64]) -> Vec<f64> {
        let mut layer_results = Vec::new();
        for node in &self.nodes {
            let mut weighted_sum = node.bias;
            for (weight, value) in node.weights.iter().zip(inputs.iter()) {
                weighted_sum += weight * value;
            }

            layer_results.push(weighted_sum.max(0.0));
        }

        layer_results
    }

    fn mutate(&mut self, rate: f64, magnitude: f64) {
        let mut rng = rand::thread_rng();
        for node in &mut self.nodes {
            for val in &mut node.weights {
                if rng.gen::<f64>() >= rate {
                    continue;
                }

                *val += rng.gen_range(-magnitude..magnitude);
            }
            if rng.gen::<f64>() < rate {
                node.bias += rng.gen_range(-magnitude..magnitude);
            }
        }
    }
}