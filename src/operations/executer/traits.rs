use std::{cell::RefCell, rc::Rc};

use crate::operations::{executer::types::Executer, planner::charts::structs::Steps};

impl Executer {
    pub fn execute_step(step: Rc<RefCell<Steps>>) -> Rc<RefCell<Steps>> {
        //Result<Rc<RefCell<Steps>>, Error> {
        step
    }
}
