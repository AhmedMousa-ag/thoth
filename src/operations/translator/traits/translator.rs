use std::{
    iter::zip,
    sync::{Arc, RwLock},
};

use crate::{
    db::controller::registerer::DbOpsRegisterer,
    err,
    operations::{
        executer::types::OperationTypes,
        planner::charts::structs::Steps,
        translator::translate::{MatricesTranslator, ScalerTranslator, VecTranslator},
    },
    structs::numerics::structs::Numeric,
    warn,
};

pub trait Translator {
    fn step(&self, step: Arc<RwLock<Steps>>) {
        // as per op_type;
        // info!("Will try to translate step");
        let op_type = step.try_read().unwrap().op_type.clone();
        match op_type {
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
    fn dot(&self);
    fn sum(&self);
    fn divide(&self);
}

impl Translator for ScalerTranslator {
    fn dot(&self) {
        let read_guard = self.step.try_read().unwrap();
        let x = match read_guard.x.as_ref().unwrap() {
            Numeric::Scaler(val) => val,
            _ => {
                let msg = "Expected Vector variant in Vector Translator";
                err!("{}",msg;panic=true);
                unreachable!("{}", msg);
            }
        };
        let y = match read_guard.y.as_ref().unwrap() {
            Numeric::Scaler(val) => val,
            _ => {
                let msg = "Expected Vector variant in Vector Translator";
                err!("{}",msg;panic=true);
                unreachable!("{}", msg);
            }
        };

        let result = y * x;
        drop(read_guard);
        self.step.try_write().unwrap().result = Some(Numeric::Scaler(result));
    }
    fn sum(&self) {
        let read_guard = self.step.try_read().unwrap();

        let x = read_guard.x.as_ref().unwrap().get_scaler_value();

        let y = read_guard.y.as_ref().unwrap().get_scaler_value();

        let result = x + y;
        drop(read_guard);
        self.step.try_write().unwrap().result = Some(Numeric::Scaler(result));
    }
    fn divide(&self) {
        let read_guard = self.step.try_read().unwrap();
        let x;
        let y;
        match read_guard.x.as_ref() {
            Some(x_step) => {
                x = x_step.get_scaler_value();
                y = read_guard.y.as_ref().unwrap().get_scaler_value();
            }
            None => {
                //if !step_ref.use_prev_res{
                let step_id = read_guard.prev_step.as_ref().unwrap().clone(); //Get last step.
                let mut prev_step =
                    DbOpsRegisterer::get_step_file(&read_guard.operation_id, &step_id);
                while prev_step.is_none() {
                    // Wait for the previous step to be available
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    prev_step = DbOpsRegisterer::get_step_file(&read_guard.operation_id, &step_id);
                }
                let prev_step = prev_step.unwrap();

                x = prev_step.result.unwrap().get_scaler_value();
                y = read_guard.y.as_ref().unwrap().get_scaler_value();
            }
        }
        let result = x / y;
        drop(read_guard);
        self.step.try_write().unwrap().result = Some(Numeric::Scaler(result));
    }
}

impl Translator for VecTranslator {
    fn dot(&self) {
        let mut result = 0.0;
        let read_guard = self.step.read().unwrap();
        let x = read_guard.x.as_ref().unwrap().get_vector_value();
        let y = read_guard.y.as_ref().unwrap().get_vector_value();

        //TODO, you might want to spawn the result in multiple threads.
        for (x_num, y_num) in zip(x, y) {
            result += y_num * x_num;
        }
        let res = Some(Numeric::Scaler(result));
        drop(read_guard);
        self.step.try_write().unwrap().result = res;
    }
    fn sum(&self) {
        let read_guard = self.step.try_read().unwrap();
        let x = match read_guard.x.as_ref().unwrap() {
            Numeric::Vector(val) => val.clone(),
            _ => {
                let msg = "Expected Vector variant in Vector Translator";
                err!("{}",msg;panic=true);
                unreachable!("{}", msg);
            }
        };

        let mut result = 0.0;
        for val in x.iter() {
            result += val;
        }
        drop(read_guard);
        self.step.try_write().unwrap().result = Some(Numeric::Scaler(result));
    }
    fn divide(&self) {
        let read_guard = self.step.try_read().unwrap();
        let x = read_guard.x.as_ref().unwrap();
        let y = read_guard.y.as_ref().unwrap();
        let result = y / x;

        drop(read_guard);
        self.step.try_write().unwrap().result = Some(result);
    }
}

//TODO MatricesTranslator
impl Translator for MatricesTranslator {
    fn dot(&self) {}
    fn sum(&self) {}
    fn divide(&self) {}
}
