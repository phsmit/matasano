extern crate rustc_serialize;

pub mod freq_scorer;
pub mod s1c01;
pub mod s1c02;
pub mod s1c05;

pub trait CanScore {
    fn score(&self, &str) -> f64;
}
