use std::sync::{Arc, RwLock};

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
        let op_id = operations_info.unwrap().try_read().unwrap()[0]
            .operation_id
            .clone();
        Self { node_duty }
    }
    fn create_translator(num: &Numeric, step: Arc<RwLock<Steps>>) -> Box<dyn Translator> {
        match num {
            Numeric::Scaler(_) => Box::new(ScalerTranslator::new(step)),
            Numeric::Vector(_) => Box::new(VecTranslator::new(step)),
            Numeric::Matrix(_) => Box::new(MatricesTranslator::new(step)),
        }
    }
    pub fn translate_step(step: Arc<RwLock<Steps>>) -> Arc<RwLock<Steps>> {
        let num = if step.try_read().unwrap().x.is_some() {
            step.try_read().unwrap().x.as_ref().unwrap().clone()
        } else {
            step.try_read().unwrap().y.as_ref().unwrap().clone()
        };
        let translator = DutiesTranslator::create_translator(&num, Arc::clone(&step));
        translator.step(Arc::clone(&step));
        Arc::clone(&step)
    }
}

//TODO handle cross Numerics operations.

pub struct ScalerTranslator {
    pub step: Arc<RwLock<Steps>>,
}
pub struct VecTranslator {
    pub step: Arc<RwLock<Steps>>,
}
pub struct MatricesTranslator {
    pub step: Arc<RwLock<Steps>>,
}

impl ScalerTranslator {
    pub fn new(step: Arc<RwLock<Steps>>) -> Self {
        Self { step: step }
    }
}

impl VecTranslator {
    pub fn new(step: Arc<RwLock<Steps>>) -> Self {
        Self { step: step }
    }
}

impl MatricesTranslator {
    pub fn new(step: Arc<RwLock<Steps>>) -> Self {
        Self { step: step }
    }
}
