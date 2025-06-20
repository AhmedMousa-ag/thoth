use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum OperationTypes {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    POW,
    SQRT, //Square Root
    ABS,
    FLOOR,
    SIN,
    COS,
    SINH,
    COSH,
    ACOS,
    TAN,
    TANH,
    SUM,
}
// pub struct Numeric {
//     x: f64,
//     y: Option<f64>,
//     op_type: OperationTypes,
// }
// impl Numeric {
//     pub fn new(x: f64, y: Option<f64>, op_type: OperationTypes) -> Self {
//         Self { x, y, op_type }
//     }
// }

// pub trait Operations {
//     fn perfom(&self) -> f64;
// }
// impl Operations for Numeric {
//     fn perfom(&self) -> f64 {
//         match self.op_type {
//             OperationTypes::ADD => self.x + self.y.unwrap_or(0.0),
//             OperationTypes::SUBTRACT => self.x - self.y.unwrap_or(0.0),
//             OperationTypes::MULTIPLY => self.x * self.y.unwrap_or(0.0),
//             OperationTypes::DIVIDE => {
//                 if self.y.unwrap_or(0.0) == 0.0 {
//                     panic!("Can't divide by zero: {}", self.y.unwrap_or(0.0))
//                 };
//                 self.x / self.y.unwrap_or(1.0)
//             }
//             OperationTypes::POW => self.x.powf(self.y.unwrap_or(1.0)),
//             OperationTypes::SQRT => self.x.sqrt(),
//             OperationTypes::ABS => self.x.abs(),
//             OperationTypes::FLOOR => self.x.floor(),
//             OperationTypes::SIN => self.x.sin(),
//             OperationTypes::SINH => self.x.sinh(),
//             OperationTypes::COS => self.x.cos(),
//             OperationTypes::COSH => self.x.cosh(),
//             OperationTypes::ACOS => self.x.acos(),
//             OperationTypes::TAN => self.x.tan(),
//             OperationTypes::TANH => self.x.tanh(),
//         }
//     }
// }
