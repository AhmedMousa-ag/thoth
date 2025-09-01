use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum OperationTypes {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DOT,
    DIVIDE,
    AVG,
    // POW,
    // SQRT, //Square Root
    // ABS,
    // FLOOR,
    // SIN,
    // COS,
    // SINH,
    // COSH,
    // ACOS,
    // TAN,
    // TANH,
    SUM,
}
impl OperationTypes {
    pub fn as_str(&self) -> &str {
        match self {
            // OperationTypes::ABS => "Absolute",
            // OperationTypes::ACOS => "ACOS",
            OperationTypes::ADD => "Add",
            OperationTypes::AVG => "Average",
            // OperationTypes::COS => "Cosin",
            // OperationTypes::COSH => "Cosh",
            OperationTypes::DIVIDE => "Divide",
            OperationTypes::DOT => "Dot",
            // OperationTypes::FLOOR => "Floor",
            OperationTypes::MULTIPLY => "Multiply",
            // OperationTypes::POW => "Power Of",
            // OperationTypes::SIN => "Sin",
            // OperationTypes::SINH => "SINH",
            // OperationTypes::SQRT => "Square",
            OperationTypes::SUBTRACT => "Subtract",
            OperationTypes::SUM => "Sum",
            // OperationTypes::TAN => "Tan",
            // OperationTypes::TANH => "Tanh",
        }
    }
}

pub struct Executer {
    // pub op_file_manager: OperationsFileManager,
}
