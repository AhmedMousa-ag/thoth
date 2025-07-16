use std::{cell::RefCell, rc::Rc};

use crate::{
    connections::channels_node_info::get_current_node_cloned,
    logger::writters::writter::OperationsFileManager,
    operations::planner::charts::structs::{NodesDuties, Steps},
};

pub enum TranslatorsTypes {
    Scalers,
    Vectors,
    Matrices,
}
impl TranslatorsTypes {
    pub fn as_str(&self) -> &str {
        match &self {
            TranslatorsTypes::Scalers => "Scaler",
            TranslatorsTypes::Vectors => "Vectors",
            TranslatorsTypes::Matrices => "Matrices",
        }
    }
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

pub struct OpsTranslator {
    node_duty: NodesDuties,
    file_manager: OperationsFileManager,
}

impl OpsTranslator {
    pub fn new(node_duty: NodesDuties) -> Self {
        let operations_info = node_duty.get(&get_current_node_cloned().id);
        let op_id = operations_info.unwrap().borrow()[0].operation_id.clone();
        let file_manager = OperationsFileManager::new(op_id).unwrap();
        Self {
            node_duty,
            file_manager,
        }
    }
}

//TODO handle cross Numerics operations.

pub struct ScalerTranslator {
    pub step: Rc<RefCell<Steps>>,
}
pub struct VecTranslator {
    pub step: Rc<RefCell<Steps>>,
}
pub struct MatricesTranslator {
    pub step: Rc<RefCell<Steps>>,
}

impl ScalerTranslator {
    fn new(step: Rc<RefCell<Steps>>) -> Self {
        Self { step: step }
    }
}

impl VecTranslator {
    fn new(step: Rc<RefCell<Steps>>) -> Self {
        Self { step: step }
    }
}

impl MatricesTranslator {
    fn new(step: Rc<RefCell<Steps>>) -> Self {
        Self { step: step }
    }
}
