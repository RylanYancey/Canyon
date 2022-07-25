
#![allow(unused_parens)]

extern crate nalgebra as na;
type Matrix = na::DMatrix<f32>;
type Vector = na::DVector<f32>;

pub mod layers;
pub use layers::*;

pub mod model;
pub use model::*;

pub mod enums;
pub use enums::*;

pub mod network;
pub use network::*;