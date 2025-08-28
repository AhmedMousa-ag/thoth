use std::{iter::zip, sync::Arc};

use tokio::sync::RwLock;

use crate::{
    db::controller::registerer::DbOpsRegisterer,
    debug, err,
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
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = step.read().await;
                let op_type = read_guard.op_type.clone();
                drop(read_guard);
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
            })
        });
    }
    fn dot(&self);
    fn sum(&self);
    fn divide(&self);
}

impl Translator for ScalerTranslator {
    fn dot(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;
                let x = read_guard.x.as_ref().unwrap();
                let y = read_guard.y.as_ref().unwrap();
                let result = y * x;
                drop(read_guard);
                self.step.write().await.result = Some(result);
            });
        });
    }
    fn sum(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;

                let x = read_guard.x.as_ref().unwrap();

                let y = read_guard.y.as_ref().unwrap();

                let result = x + y;
                drop(read_guard);

                self.step.write().await.result = Some(result);
            });
        });
    }
    fn divide(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;

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
                        while prev_step.is_none() || prev_step.as_ref().unwrap().result.is_none() {
                            // Wait for the previous step to be available
                            std::thread::sleep(std::time::Duration::from_millis(1));
                            prev_step =
                                DbOpsRegisterer::get_step_file(&read_guard.operation_id, &step_id);
                        }
                        let prev_step = prev_step.unwrap();
                        debug!("Using previous step result: {}", prev_step);
                        x = prev_step.result.unwrap().get_scaler_value();
                        y = read_guard.y.as_ref().unwrap().get_scaler_value();
                    }
                }
                let result = x / y;
                drop(read_guard);

                self.step.write().await.result = Some(Numeric::Scaler(result));
            });
        });
    }
}

impl Translator for VecTranslator {
    fn dot(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let mut result = 0.0;
                let read_guard = self.step.read().await;
                let x = read_guard.x.as_ref().unwrap().get_vector_value();
                let y = read_guard.y.as_ref().unwrap().get_vector_value();

                //TODO, you might want to spawn the result in multiple threads.
                for (x_num, y_num) in zip(x, y) {
                    result += y_num * x_num;
                }
                let res = Some(Numeric::Scaler(result));
                drop(read_guard);

                self.step.write().await.result = res;
            });
        });
    }
    fn sum(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;
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

                self.step.write().await.result = Some(Numeric::Scaler(result));
            });
        });
    }
    fn divide(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;
                let x = read_guard.x.as_ref().unwrap();
                let y = read_guard.y.as_ref().unwrap();
                let result = y / x;

                drop(read_guard);

                self.step.write().await.result = Some(result);
            });
        });
    }
}

//TODO MatricesTranslator
impl Translator for MatricesTranslator {
    fn dot(&self) {}
    fn sum(&self) {}
    fn divide(&self) {}
}
