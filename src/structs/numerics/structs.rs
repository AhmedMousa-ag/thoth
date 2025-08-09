use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum Numeric {
    Scaler(f64),
    Vector(Vec<f64>),
    Matrix(Vec<Vec<f64>>),
}
