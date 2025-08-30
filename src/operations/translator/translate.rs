use tokio::sync::RwLock;

use crate::{
    connections::channels_node_info::get_current_node_cloned,
    operations::{
        planner::charts::structs::{NodesDuties, OperationInfo, Steps},
        translator::traits::translator::Translator,
    },
    structs::numerics::structs::Numeric,
};
use std::sync::Arc;

pub struct DutiesTranslator {
    pub node_duty: Vec<OperationInfo>,
}

impl DutiesTranslator {
    //TODO this function seems useless as all the methodes are static. Unless you're going to do a check before excuting a step. Consider deleting it, seems uselss.
    pub fn new(nodes_duty: NodesDuties) -> Option<Self> {
        let operations_info: Option<&Vec<OperationInfo>> =
            nodes_duty.get(&get_current_node_cloned().id);
        match operations_info {
            Some(node_duty) => Some(Self {
                node_duty: node_duty.clone(),
            }),
            None => None,
        }
    }
    fn create_translator(num: &Numeric, step: Arc<RwLock<Steps>>) -> Box<dyn Translator> {
        match num {
            Numeric::Scaler(_) => Box::new(ScalerTranslator::new(step)),
            Numeric::Vector(_) => Box::new(VecTranslator::new(step)),
            Numeric::Matrix(_) => Box::new(MatricesTranslator::new(step)),
        }
    }
    pub async fn translate_step(step: Arc<RwLock<Steps>>) -> Arc<RwLock<Steps>> {
        let read_guard = step.read().await;
        let num = match read_guard.x.as_ref() {
            Some(x) => x.clone(),
            None => read_guard.y.as_ref().unwrap().clone(),
        };
        drop(read_guard);

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
