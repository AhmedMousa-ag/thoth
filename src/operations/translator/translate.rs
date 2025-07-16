use std::{cell::RefCell, rc::Rc};

use crate::{
    connections::channels_node_info::get_current_node_cloned,
    operations::{
        planner::charts::structs::{NodesDuties, Numeric, Steps},
        translator::traits::translator::Translator,
    },
};

pub struct DutiesTranslator {
    node_duty: NodesDuties,
}

impl DutiesTranslator {
    pub fn new(node_duty: NodesDuties) -> Self {
        let operations_info = node_duty.get(&get_current_node_cloned().id);
        let op_id = operations_info.unwrap().borrow()[0].operation_id.clone();
        Self { node_duty }
    }
    fn create_translator(num: &Numeric, step: Rc<RefCell<Steps>>) -> Box<dyn Translator> {
        match num {
            Numeric::Scaler(_) => Box::new(ScalerTranslator::new(step)),
            Numeric::Vector(_) => Box::new(VecTranslator::new(step)),
            Numeric::Matrix(_) => Box::new(MatricesTranslator::new(step)),
        }
    }
    pub fn translate_step(step: Rc<RefCell<Steps>>) -> Rc<RefCell<Steps>> {
        let borrowed_step = step.borrow();
        let num = if borrowed_step.x.is_some() {
            borrowed_step.x.as_ref().unwrap()
        } else {
            borrowed_step.y.as_ref().unwrap()
        };
        let translator = DutiesTranslator::create_translator(num, step.clone());
        translator.step(step.clone());
        step.clone()
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
    pub fn new(step: Rc<RefCell<Steps>>) -> Self {
        Self { step: step }
    }
}

impl VecTranslator {
    pub fn new(step: Rc<RefCell<Steps>>) -> Self {
        Self { step: step }
    }
}

impl MatricesTranslator {
    pub fn new(step: Rc<RefCell<Steps>>) -> Self {
        Self { step: step }
    }
}
