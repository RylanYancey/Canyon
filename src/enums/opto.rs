
#[derive(Copy, Clone)]
pub enum Opto {
    Momentum(f32),
    Linear,
    Adam(f32, f32),
}