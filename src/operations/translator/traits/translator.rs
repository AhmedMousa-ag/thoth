use std::{
    cell::RefCell,
    iter::zip,
    rc::Rc,
    sync::{Arc, RwLock},
};

use crate::{
    err,
    logger::writters::writter::OperationsFileManager,
    operations::{
        executer::types::OperationTypes,
        planner::charts::structs::{Numeric, Steps},
        translator::translate::{MatricesTranslator, ScalerTranslator, VecTranslator},
    },
    warn,
};

pub trait Translator {
    fn step(&self, step: Arc<RwLock<Steps>>) {
        // as per op_type;

        match step.try_read().unwrap().op_type {
            OperationTypes::DOT => {
                self.dot();
            }
            OperationTypes::SUM => {
                self.sum();
            }
            OperationTypes::DIVIDE => {
                self.divide();
            }
            _ => {
                warn!("Other operations not supported yet");
            }
        }
    }
    fn dot(&self) -> Arc<RwLock<Steps>>;
    fn sum(&self) -> Arc<RwLock<Steps>>;
    fn divide(&self) -> Arc<RwLock<Steps>>;
}

impl Translator for ScalerTranslator {
    fn dot(&self) -> Arc<RwLock<Steps>> {
        let step_ref = self.step.try_read().unwrap();
        let x = match step_ref.x.as_ref().unwrap() {
            Numeric::Scaler(val) => val,
            _ => {
                let msg = "Expected Vector variant in Vector Translator";
                err!("{}",msg;panic=true);
                unreachable!("{}", msg);
            }
        };
        let y = match step_ref.y.as_ref().unwrap() {
            Numeric::Scaler(val) => val,
            _ => {
                let msg = "Expected Vector variant in Vector Translator";
                err!("{}",msg;panic=true);
                unreachable!("{}", msg);
            }
        };

        let result = x.as_ref() * y.as_ref();
        self.step.try_write().unwrap().result = Some(Numeric::Scaler(Box::new(result)));
        self.step.clone()
    }
    fn sum(&self) -> Arc<RwLock<Steps>> {
        let step_ref = self.step.try_read().unwrap();

        let x = step_ref.x.as_ref().unwrap().get_scaler_value();

        let y = step_ref.y.as_ref().unwrap().get_scaler_value();

        let result = x.as_ref() + y.as_ref();

        self.step.try_write().unwrap().result = Some(Numeric::Scaler(Box::new(result)));
        self.step.clone()
    }
    fn divide(&self) -> Arc<RwLock<Steps>> {
        let step_ref = self.step.try_read().unwrap();
        let x;
        let y;
        match step_ref.x.as_ref() {
            Some(x_step) => {
                x = *x_step.get_scaler_value();
                y = *step_ref.y.as_ref().unwrap().get_scaler_value();
            }
            None => {
                //if !step_ref.use_prev_res{
                let step_id = step_ref.prev_step.as_ref().unwrap(); //Get last step.
                let prev_step =
                    OperationsFileManager::load_step_file(&step_ref.operation_id, step_id);
                x = *prev_step.result.unwrap().get_scaler_value();
                y = *step_ref.y.as_ref().unwrap().get_scaler_value();
            }
        }
        let result = y * x;
        self.step.try_write().unwrap().result = Some(Numeric::Scaler(Box::new(result)));
        self.step.clone()
    }
}

impl Translator for VecTranslator {
    fn dot(&self) -> Arc<RwLock<Steps>> {
        let step = self.step.clone();
        let x = step.try_read().unwrap().x.as_ref().unwrap().get_vector_value();
        let y = step.try_read().unwrap().y.as_ref().unwrap().get_vector_value();

        let mut result = 0.0;
        //TODO, you might want to spawn the result in multiple threads.
        for (x_num, y_num) in zip(x, y) {
            result += y_num.as_ref() * x_num.as_ref();
        }
        step.try_write().unwrap().result = Some(Numeric::Scaler(Box::new(result)));
        step.clone()
    }
    fn sum(&self) -> Arc<RwLock<Steps>> {
        let step_ref = self.step.try_read().unwrap();

        let x = match step_ref.x.as_ref().unwrap() {
            Numeric::Vector(val) => val,
            _ => {
                let msg = "Expected Vector variant in Vector Translator";
                err!("{}",msg;panic=true);
                unreachable!("{}", msg);
            }
        };

        let mut result = 0.0;
        for val in x.iter() {
            result += val.as_ref();
        }

        self.step.try_write().unwrap().result = Some(Numeric::Scaler(Box::new(result)));
        self.step.clone()
    }
    fn divide(&self) -> Arc<RwLock<Steps>> {
        self.step.clone()
    }
}

//TODO MatricesTranslator
impl Translator for MatricesTranslator {
    fn dot(&self) -> Arc<RwLock<Steps>> {
        self.step.clone()
    }
    fn sum(&self) -> Arc<RwLock<Steps>> {
        self.step.clone()
    }
    fn divide(&self) -> Arc<RwLock<Steps>> {
        self.step.clone()
    }
}
