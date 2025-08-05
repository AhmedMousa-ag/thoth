use std::env;

use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder,
};

use crate::db::entities::{nodes_duties, operations, steps, synced_ops};
use crate::db::{
    entities::{
        nodes_duties::{ActiveModel as NodesDutiesModels, Entity as NodesDuties},
        operations::{ActiveModel as OperationsModel, Entity as Operations},
        steps::{ActiveModel as StepsModel, Entity as Steps},
        synced_ops::{ActiveModel as SyncedOpsModel, Entity as SyncedOps},
    },
    sqlite::get_db_connection,
};
use crate::err;
use crate::errors::thot_errors::ThothErrors;

use chrono::{self, DateTime, Utc};
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
            res_pos: ActiveValue::NotSet,
            res_type: ActiveValue::NotSet,
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
    pub fn new(op_id: String, node_id: String, step_id: String) -> NodesDutiesModels {
        NodesDutiesModels {
            op_id: ActiveValue::Set(op_id),
            node_id: ActiveValue::Set(node_id),
            is_finished: ActiveValue::Set(false),
            step_id: ActiveValue::Set(step_id),
        }
    }
    pub fn find_all_duties(op_id: String) -> Vec<nodes_duties::Model> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async {
                    let db = get_db_connection().await;
                    NodesDuties::find()
                        .filter(nodes_duties::Column::OpId.eq(op_id))
                        .all(db)
                        .await
                })
                .unwrap()
        })
    }
}

pub struct SqlSyncedOps {}
impl SQLiteDBTraits<SyncedOps, SyncedOpsModel> for SqlSyncedOps {
    fn find_by_id(id: String) -> Option<<SyncedOps as EntityTrait>::Model> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                SyncedOps::find()
                    .filter(synced_ops::Column::SyncedId.eq(id))
                    .one(db)
                    .await
                    .unwrap()
            })
        })
    }
}
impl SqlSyncedOps {
    pub fn new(date_from: DateTime<Utc>, date_to: DateTime<Utc>) -> synced_ops::ActiveModel {
        SyncedOpsModel {
            synced_id: ActiveValue::NotSet,
            from_date: ActiveValue::Set(date_from),
            to_date: ActiveValue::Set(date_to),
            ops_id: ActiveValue::NotSet,
            is_finished: ActiveValue::Set(false),
        }
    }
    pub fn find_by_dates(
        date_from: DateTime<Utc>,
        date_to: DateTime<Utc>,
        is_finished: Option<bool>,
    ) -> Option<synced_ops::Model> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                let mut query = SyncedOps::find_by_id((date_from, date_to));
                match is_finished {
                    Some(finished) => {
                        query = query.filter(synced_ops::Column::IsFinished.eq(finished))
                    }
                    None => {}
                };
                query.one(db).await.unwrap()
            })
        })
    }

    pub fn find_by_date_from(
        date_from: DateTime<Utc>,
        is_finished: Option<bool>,
    ) -> Result<Vec<synced_ops::Model>, DbErr> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                let mut query =
                    SyncedOps::find().filter(synced_ops::Column::FromDate.gte(date_from));
                match is_finished {
                    Some(finished) => {
                        query = query.filter(synced_ops::Column::IsFinished.eq(finished))
                    }
                    None => {}
                };
                query.all(db).await
            })
        })
    }
    pub fn find_by_date_to(
        date_to: DateTime<Utc>,
        is_finished: Option<bool>,
    ) -> Result<Vec<synced_ops::Model>, DbErr> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                let mut query = SyncedOps::find().filter(synced_ops::Column::ToDate.gte(date_to));
                match is_finished {
                    Some(finished) => {
                        query = query.filter(synced_ops::Column::IsFinished.eq(finished))
                    }
                    None => {}
                };
                query.all(db).await
            })
        })
    }
    //TODO by operations.
    pub fn find_by_operation(
        op_id: String,
        is_finished: Option<bool>,
    ) -> Result<Vec<synced_ops::Model>, DbErr> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                let mut query = SyncedOps::find().filter(synced_ops::Column::OpsId.eq(op_id));
                match is_finished {
                    Some(finished) => {
                        query = query.filter(synced_ops::Column::IsFinished.eq(finished))
                    }
                    None => {}
                };
                query.all(db).await
            })
        })
    }

    pub fn get_latest_finished() -> Option<synced_ops::Model> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                let query: Result<Option<synced_ops::Model>, DbErr> = SyncedOps::find()
                    .filter(synced_ops::Column::IsFinished.eq(true))
                    .order_by_desc(synced_ops::Column::IsFinished)
                    .one(db)
                    .await;
                match query {
                    Ok(res) => res,
                    Err(e) => {
                        err!("Get latest synced operation {}", ThothErrors::from(e));
                        None
                    }
                }
            })
        })
    }
}
