use std::{cell::RefCell, iter::zip, rc::Rc};

use crate::{
    err,
    operations::{
        executer::types::OperationTypes,
        planner::charts::structs::{Numeric, Steps},
        translator::translate::{MatricesTranslator, ScalerTranslator, VecTranslator},
    },
    warn,
};

pub trait Translator {
    fn step(&self, step: Rc<RefCell<Steps>>) {
        // as per op_type;

        match step.borrow().op_type {
            OperationTypes::DOT => {
                self.dot();
            }
            OperationTypes::SUM => {
                self.sum();
            }
            _ => {
                warn!("Other operations not supported yet");
            }
        }
    }
    fn dot(&self) -> Rc<RefCell<Steps>>;
    fn sum(&self) -> Rc<RefCell<Steps>>;
}

impl Translator for ScalerTranslator {
    fn dot(&self) -> Rc<RefCell<Steps>> {
        let step_ref = self.step.borrow();
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
        self.step.borrow_mut().result = Some(Numeric::Scaler(Box::new(result)));
        self.step.clone()
    }
    fn sum(&self) -> Rc<RefCell<Steps>> {
        let step_ref = self.step.borrow();

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

        let result = x.as_ref() + y.as_ref();

        self.step.borrow_mut().result = Some(Numeric::Scaler(Box::new(result)));
        self.step.clone()
    }
}

impl Translator for VecTranslator {
    fn dot(&self) -> Rc<RefCell<Steps>> {
        let step_ref = self.step.borrow();
        let x = match step_ref.x.as_ref().unwrap() {
            Numeric::Vector(val) => val,
            _ => {
                let msg = "Expected Vector variant in Vector Translator";
                err!("{}",msg;panic=true);
                unreachable!("{}", msg);
            }
        };
        let y = match step_ref.y.as_ref().unwrap() {
            Numeric::Vector(val) => val,
            _ => {
                let msg = "Expected Vector variant in Vector Translator";
                err!("{}",msg;panic=true);
                unreachable!("{}", msg);
            }
        };

        let mut result = 0.0;
        //TODO, you might want to spawn the result in multiple threads.
        for (x_num, y_num) in zip(x, y) {
            result += y_num.as_ref() * x_num.as_ref();
        }
        self.step.borrow_mut().result = Some(Numeric::Scaler(Box::new(result)));
        self.step.clone()
    }
    fn sum(&self) -> Rc<RefCell<Steps>> {
        let step_ref = self.step.borrow();

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

        self.step.borrow_mut().result = Some(Numeric::Scaler(Box::new(result)));
        self.step.clone()
    }
}

//TODO MatricesTranslator
impl Translator for MatricesTranslator {
    fn dot(&self) -> Rc<RefCell<Steps>> {
        self.step.clone()
    }
    fn sum(&self) -> Rc<RefCell<Steps>> {
        self.step.clone()
    }
}
