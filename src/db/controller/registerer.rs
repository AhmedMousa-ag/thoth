use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use tokio::spawn;

use crate::{
    db::{
        controller::traits::{
            SQLiteDBTraits, SqlNodesDuties, SqlOperations, SqlSteps, SqlSyncedOps,
        },
        entities::nodes_duties::ActiveModel as NodesDutiesActiveModel,
    },
    err,
    errors::thot_errors::ThothErrors,
    operations::planner::charts::structs::NodesOpsMsg,
};

pub struct DbOpsRegisterer {}
impl DbOpsRegisterer {
    pub fn new_syncer(date_from: DateTime<Utc>, date_to: DateTime<Utc>, thread: bool) {
        let fnc = move || {
            if let Err(e) = SqlSyncedOps::insert_row(SqlSyncedOps::new(date_from, date_to)) {
                err!("new synced operation {}", ThothErrors::from(e))
            };
        };
        if thread {
            spawn(async move {
                fnc();
            });
        } else {
            fnc();
        }
    }
    pub fn new_operation(operation_id: String, thread: bool) {
        let fnc = move || {
            if let Err(e) = SqlOperations::insert_row(SqlOperations::new(operation_id)) {
                err!("new operations {}", ThothErrors::from(e))
            };
        };
        if thread {
            spawn(async move {
                fnc();
            });
        } else {
            fnc();
        }
    }
    pub fn new_step(operation_id: String, step_id: String, use_prev_res: bool, thread: bool) {
        let fnc = move || {
            let mut model = SqlSteps::new(step_id.clone(), operation_id.clone());
            model.use_prev_res = Set(use_prev_res);
            if let Err(e) = SqlSteps::insert_row(model) {
                err!("new step {}", ThothErrors::from(e))
            };
        };
        if thread {
            spawn(async move {
                fnc();
            });
        } else {
            fnc();
        }
    }
    pub fn finished_step() {}
    pub fn new_duty(node_id: String, operation_id: String, step_id: String, thread: bool) {
        let fnc = move || {
            let sql_duty = NodesDutiesActiveModel {
                node_id: Set(node_id.clone()),
                op_id: Set(operation_id.clone()),
                step_id: Set(step_id.clone()),
                is_finished: Set(false),
            };

            if let Err(e) = SqlNodesDuties::insert_row(sql_duty) {
                err!("new node duty {}", ThothErrors::from(e))
            };
        };
        if thread {
            spawn(async move {
                fnc();
            });
        } else {
            fnc();
        }
    }
    pub fn finished_duty(step_id: String, thread: bool) {
        let fnc = move || {
            let mut sql_duty = NodesDutiesActiveModel {
                step_id: Set(step_id.clone()),
                ..Default::default()
            };
            sql_duty.is_finished = Set(true);
            if let Err(e) = SqlNodesDuties::update_row(sql_duty) {
                err!("Marking duty as finished: {}", ThothErrors::from(e));
            };
        };
        if thread {
            spawn(async move {
                fnc();
            });
        } else {
            fnc();
        }
    }
    /// Register both a step and a duty in one funciton
    pub fn new_step_duty(
        node_id: String,
        operation_id: String,
        step_id: String,
        use_prev_res: bool,
        thread: bool,
    ) {
        DbOpsRegisterer::new_step(operation_id.clone(), step_id.clone(), use_prev_res, thread);
        DbOpsRegisterer::new_duty(node_id, operation_id, step_id, thread);
    }
    pub fn new_duties(duties: NodesOpsMsg, thread: bool) {
        for (node_id, ops_info) in duties.nodes_duties {
            for ops_infos in ops_info.try_read().unwrap().clone() {
                DbOpsRegisterer::new_duty(
                    node_id.clone(),
                    ops_infos.operation_id,
                    ops_infos.step_id,
                    thread,
                );
            }
        }
    }
}
