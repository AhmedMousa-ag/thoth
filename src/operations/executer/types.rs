use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::err;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum OperationTypes {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DOT,
    DIVIDE,
    AVG,
    ORDERLIST,
    MAX,
    MIN,
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
            OperationTypes::ORDERLIST => "Order List",
            OperationTypes::MAX => "Max",
            OperationTypes::MIN => "Min",
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum OperationsHelper {
    ASCENDING,
    DESCENDING,
}
impl OperationsHelper {
    pub fn as_str(&self) -> &str {
        match self {
            OperationsHelper::ASCENDING => "Ascending",
            OperationsHelper::DESCENDING => "Descending",
        }
    }
}
impl Into<String> for OperationsHelper {
    fn into(self) -> String {
        match self {
            OperationsHelper::ASCENDING => "Ascending".to_string(),
            OperationsHelper::DESCENDING => "Descending".to_string(),
        }
    }
}
impl Into<OperationsHelper> for String {
    fn into(self) -> OperationsHelper {
        match self.as_str() {
            "Ascending" => OperationsHelper::ASCENDING,
            "Descending" => OperationsHelper::DESCENDING,
            _ => {
                err!("Invalid string for OperationsHelper");
                unreachable!()
            }
        }
    }
}

pub struct Executer {
    // pub op_file_manager: OperationsFileManager,
}
