use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    db::controller::registerer::DbOpsRegisterer,
    operations::{
        executer::types::{OperationTypes, OperationsHelper},
        planner::charts::structs::Steps,
        translator::translate::{MatricesTranslator, ScalerTranslator, VecTranslator},
    },
    structs::numerics::structs::{Numeric, SharedNumeric},
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
                    OperationTypes::AVG => {
                        self.avg();
                    }
                    OperationTypes::ORDERLIST => {
                        self.order_list();
                    }
                    OperationTypes::MAX => {
                        self.max();
                    }
                    OperationTypes::MIN => {
                        self.min();
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
    fn avg(&self);
    fn order_list(&self);
    fn max(&self) {}
    fn min(&self) {}
}

impl Translator for ScalerTranslator {
    fn dot(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;
                let x = read_guard.x.as_ref().unwrap().0.clone();
                let y = read_guard.y.as_ref().unwrap().0.clone();
                drop(read_guard);
                let result = y.read().await.get_scaler_value() * x.read().await.get_scaler_value();
                self.step.write().await.result = Some(SharedNumeric::new(Numeric::Scaler(result)));
            });
        });
    }
    fn sum(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;
                let x = read_guard.x.as_ref().unwrap().0.clone();
                let y = read_guard.y.as_ref().unwrap().0.clone();
                drop(read_guard);
                let result = x.read().await.get_scaler_value() + y.read().await.get_scaler_value();
                self.step.write().await.result = Some(SharedNumeric::new(Numeric::Scaler(result)));
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
                        x = x_step.0.read().await.get_scaler_value();
                        y = read_guard
                            .y
                            .as_ref()
                            .unwrap()
                            .0
                            .read()
                            .await
                            .get_scaler_value();
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
                        x = prev_step.result.unwrap().0.read().await.get_scaler_value();
                        y = read_guard
                            .y
                            .as_ref()
                            .unwrap()
                            .0
                            .read()
                            .await
                            .get_scaler_value();
                    }
                }
                drop(read_guard);
                let result = x / y;

                self.step.write().await.result = Some(SharedNumeric::new(Numeric::Scaler(result)));
            });
        });
    }
    fn avg(&self) {}
    fn order_list(&self) {}
    fn max(&self) {}
    fn min(&self) {}
}

impl Translator for VecTranslator {
    fn dot(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;
                let x = read_guard.x.as_ref().unwrap().clone();
                let y = read_guard.y.as_ref().unwrap().clone();
                drop(read_guard);
                let y = y.0.read().await;
                let x = x.0.read().await;

                let x = x.get_vector_value();
                let y = y.get_vector_value();

                // Divide and conquer dot product for large vectors
                fn dot_product_divide_and_conquer(x: &[f64], y: &[f64]) -> f64 {
                    const THRESHOLD: usize = 1024;
                    if x.len() <= THRESHOLD {
                        x.iter()
                            .zip(y.iter())
                            .map(|(x_num, y_num)| x_num * y_num)
                            .sum()
                    } else {
                        let mid = x.len() / 2;
                        let (x_left, x_right) = x.split_at(mid);
                        let (y_left, y_right) = y.split_at(mid);
                        let left = dot_product_divide_and_conquer(x_left, y_left);
                        let right = dot_product_divide_and_conquer(x_right, y_right);
                        left + right
                    }
                }

                let result = dot_product_divide_and_conquer(&x, &y);

                self.step.write().await.result = Some(SharedNumeric::new(Numeric::Scaler(result)));
            });
        });
    }
    fn sum(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;
                let x = read_guard.x.as_ref().unwrap().clone();
                drop(read_guard);
                let x = x.0.read().await;
                let x = x.get_vector_value();

                let mut result = 0.0;
                for val in x.iter() {
                    result += val;
                }
                self.step.write().await.result = Some(SharedNumeric::new(Numeric::Scaler(result)));
            });
        });
    }
    fn divide(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;
                let x = read_guard.x.as_ref().unwrap().clone();
                let y = read_guard.y.as_ref().unwrap().clone();
                drop(read_guard);
                let x = x.0.read().await;
                let y = y.0.read().await;

                let x = x.get_vector_value();
                let y = y.get_vector_value();
                // Element-wise division of two vectors
                let result: Vec<f64> = x
                    .iter()
                    .zip(y.iter())
                    .map(|(x_val, y_val)| x_val / y_val)
                    .collect();

                self.step.write().await.result = Some(SharedNumeric::new(Numeric::Vector(result)));
            });
        });
    }
    fn avg(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;
                let x = read_guard.x.as_ref().unwrap().clone();
                drop(read_guard);
                let x = x.0.read().await;
                let x = x.get_vector_value();

                let result = x.iter().sum::<f64>() / (x.len() as f64);
                self.step.write().await.result = Some(SharedNumeric::new(Numeric::Scaler(result)));
            });
        });
    }
    fn order_list(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;
                let x = read_guard.x.as_ref().unwrap().clone();
                let order_type: OperationsHelper = read_guard
                    .extra_info
                    .as_ref()
                    .unwrap()
                    .clone()
                    .helper_string
                    .unwrap()
                    .into();
                drop(read_guard);
                let x = x.0.read().await;
                let x = x.get_vector_value();
                let mut result = x.clone();

                match order_type {
                    OperationsHelper::DESCENDING => {
                        result.sort_by(|a, b| b.partial_cmp(a).unwrap())
                    } // Descending
                    // Ascending
                    OperationsHelper::ASCENDING => result.sort_by(|a, b| a.partial_cmp(b).unwrap()),
                };
                self.step.write().await.result = Some(SharedNumeric::new(Numeric::Vector(result)));
            });
        });
    }
    fn max(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;
                let x = read_guard.x.as_ref().unwrap().clone();
                drop(read_guard);
                let x = x.0.read().await;
                let x = x.get_vector_value();
                let result = x
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(&0.0)
                    .clone();
                self.step.write().await.result = Some(SharedNumeric::new(Numeric::Scaler(result)));
            });
        });
    }
    fn min(&self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let read_guard = self.step.read().await;
                let x = read_guard.x.as_ref().unwrap().clone();
                drop(read_guard);
                let x = x.0.read().await;
                let x = x.get_vector_value();
                let result = x
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(&0.0)
                    .clone();
                self.step.write().await.result = Some(SharedNumeric::new(Numeric::Scaler(result)));
            });
        });
    }
}

//TODO MatricesTranslator
impl Translator for MatricesTranslator {
    fn dot(&self) {}
    fn sum(&self) {}
    fn divide(&self) {}
    fn avg(&self) {}
    fn order_list(&self) {}
    fn max(&self) {}
    fn min(&self) {}
}
