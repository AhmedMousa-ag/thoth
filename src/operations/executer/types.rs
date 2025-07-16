use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::logger::writters::writter::OperationsFileManager;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum OperationTypes {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DOT,
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
impl OperationTypes {
    pub fn as_str(&self) -> &str {
        match self {
            OperationTypes::ABS => "Absolute",
            OperationTypes::ACOS => "ACOS",
            OperationTypes::ADD => "Add",
            OperationTypes::COS => "Cosin",
            OperationTypes::COSH => "Cosh",
            OperationTypes::DIVIDE => "Divide",
            OperationTypes::DOT => "Dot",
            OperationTypes::FLOOR => "Floor",
            OperationTypes::MULTIPLY => "Multiply",
            OperationTypes::POW => "Power Of",
            OperationTypes::SIN => "Sin",
            OperationTypes::SINH => "SINH",
            OperationTypes::SQRT => "Square",
            OperationTypes::SUBTRACT => "Subtract",
            OperationTypes::SUM => "Sum",
            OperationTypes::TAN => "Tan",
            OperationTypes::TANH => "Tanh",
        }
    }
}

pub struct Executer {
    pub op_file_manager: OperationsFileManager,
}

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
//         }
//     }

//     fn perform_vector(&self)->Vec<Box<f64>>{

//     }
//     fn perform_matrix(&self)->Vec<Vec<Box<f64>>>{

//     }
// }
