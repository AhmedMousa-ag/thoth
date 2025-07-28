use std::env;

use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel};

use crate::db::entities::{nodes_duties, operations, steps};
use crate::db::{
    entities::{
        nodes_duties::{ActiveModel as NodesDutiesModels, Entity as NodesDuties},
        operations::{ActiveModel as OperationsModel, Entity as Operations},
        steps::{ActiveModel as StepsModel, Entity as Steps},
    },
    sqlite::get_db_connection,
};
use crate::debug;

use chrono;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

pub trait SQLiteDBTraits<T, A>
where
    T: EntityTrait,
    A: ActiveModelTrait<Entity = T> + ActiveModelBehavior + Send,
    <T as EntityTrait>::Model: IntoActiveModel<A>,
{
    fn find_by_id(id: String) -> Option<<T as EntityTrait>::Model>;
    fn get_all() -> Vec<<T as EntityTrait>::Model> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                let res = T::find().all(db).await.unwrap();
                res
            })
        })
    }

    fn insert_row(row: A) -> Result<<T as sea_orm::EntityTrait>::Model, sea_orm::DbErr> {
        debug!("Inserting row into the database: {:?}", row);
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                row.insert(db).await
            })
        })
    }
    fn update_row(row: A) -> Result<<T as sea_orm::EntityTrait>::Model, sea_orm::DbErr> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                row.update(db).await
            })
        })
    }
}

pub struct SqlSteps {}
impl SQLiteDBTraits<Steps, StepsModel> for SqlSteps {
    fn find_by_id(id: String) -> Option<steps::Model> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                Steps::find_by_id(id).one(db).await.unwrap()
            })
        })
    }
}
impl SqlSteps {
    pub fn new(step_id: String, op_id: String) -> StepsModel {
        let file_path = env::current_dir()
            .unwrap()
            .join(&op_id)
            .to_str()
            .unwrap()
            .to_string();
        StepsModel {
            op_id: ActiveValue::Set(op_id),
            step_id: ActiveValue::Set(step_id),
            file_path: ActiveValue::Set(file_path),
            is_finished: ActiveValue::Set(false),
            result: ActiveValue::NotSet,
        }
    }
}

pub struct SqlOperations {}
impl SQLiteDBTraits<Operations, OperationsModel> for SqlOperations {
    fn find_by_id(id: String) -> Option<operations::Model> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                Operations::find_by_id(id).one(db).await.unwrap()
            })
        })
    }
}
impl SqlOperations {
    pub fn new(op_id: String) -> OperationsModel {
        OperationsModel {
            op_id: ActiveValue::Set(op_id),
            exec_date: ActiveValue::Set(chrono::offset::Utc::now()),
            is_finished: ActiveValue::Set(false),
        }
    }
}

pub struct SqlNodesDuties {}
impl SQLiteDBTraits<NodesDuties, NodesDutiesModels> for SqlNodesDuties {
    fn find_by_id(id: String) -> Option<nodes_duties::Model> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                NodesDuties::find_by_id(id).one(db).await.unwrap()
            })
        })
    }
}
impl SqlNodesDuties {
    pub fn new(op_id: String, node_id: String) -> NodesDutiesModels {
        NodesDutiesModels {
            op_id: ActiveValue::Set(op_id),
            node_id: ActiveValue::Set(node_id),
            is_finished: ActiveValue::Set(false),
        }
    }
}
