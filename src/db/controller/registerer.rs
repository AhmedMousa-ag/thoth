use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::RwLock;

use crate::{
    db::{
        controller::traits::{SQLiteDBTraits, SqlNodesDuties, SqlSyncedOps},
        entities::nodes_duties::ActiveModel as NodesDutiesActiveModel,
    },
    err,
    errors::thot_errors::ThothErrors,
    logger::writters::writter::OperationsFileManager,
    operations::planner::charts::structs::{NodesOpsMsg, OperationFile, Steps},
    warn,
};
pub struct FileRegisterer {}
impl FileRegisterer {
    pub fn new_operation(operation_id: String, thread: bool) {
        let fnc = move || {
            let mut op_file = OperationsFileManager::new(&operation_id);
            let execution_date: DateTime<Utc> = Utc::now();
            match op_file.create_operation_file(
                OperationFile {
                    operation_id,
                    result: None,
                    execution_date,
                },
                false,
            ) {
                Ok(_) => {}
                Err(e) => {
                    warn!("Failed to create operation file: {}", ThothErrors::from(e));
                }
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

    pub async fn new_step(step: Arc<RwLock<Steps>>, thread: bool) {
        let read_guard = step.read().await;
        let op_id = read_guard.operation_id.clone();
        let step_id = read_guard.step_id.clone();
        let step_str_lines = serde_json::to_string(&*read_guard).unwrap();
        drop(read_guard);
        let fnc = move || match OperationsFileManager::new(&op_id).write_step(
            step_str_lines,
            op_id,
            step_id,
            false,
        ) {
            Ok(_) => {}
            Err(e) => {
                err!("Failed to write step file: {}", ThothErrors::from(e));
            }
        };
        if thread {
            spawn(async move {
                fnc();
            });
        } else {
            fnc();
        }
    }
}
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
        FileRegisterer::new_operation(operation_id, thread);
    }
    pub fn get_operation_file(operation_id: &str) -> Option<OperationFile> {
        OperationsFileManager::load_operation_file(operation_id)
    }
    pub fn get_operation_by_date(
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Vec<OperationFile> {
        OperationsFileManager::load_operations_by_date(start_date, end_date)
    }
    pub fn get_step_file(operation_id: &str, step_id: &str) -> Option<Steps> {
        match OperationsFileManager::load_step_file(operation_id, step_id) {
            Ok(step) => Some(step),
            Err(e) => {
                err!(
                    "Faild to load step file for step id {}: {}",
                    step_id,
                    ThothErrors::from(e)
                );
                None
            }
        }
    }
    pub fn get_steps_by_op_id(operation_id: &str) -> Vec<Steps> {
        OperationsFileManager::load_steps_by_op_id(operation_id)
    }
    pub async fn new_step(step: Arc<RwLock<Steps>>, thread: bool) {
        FileRegisterer::new_step(step, thread).await;
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
    pub async fn new_step_duty(
        node_id: String,
        operation_id: String,
        step: Arc<RwLock<Steps>>,
        thread: bool,
    ) {
        DbOpsRegisterer::new_step(step.clone(), thread).await;
        DbOpsRegisterer::new_duty(
            node_id,
            operation_id,
            step.read().await.step_id.clone(),
            thread,
        );
    }
    pub fn new_duties(duties: &NodesOpsMsg, thread: bool) {
        for (node_id, ops_info) in duties.nodes_duties.clone() {
            for ops_infos in ops_info.clone() {
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
