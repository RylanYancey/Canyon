
pub trait Layer {
    fn forward(&mut self);
    fn backward(&mut self);
    fn update(&mut self);
}