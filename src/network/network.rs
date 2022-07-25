
use super::*;

pub struct Network {
    layers: Vec<Box<dyn Layer>>,
}