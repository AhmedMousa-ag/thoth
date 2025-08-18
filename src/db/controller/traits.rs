use std::env;

use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder,
};

use crate::db::entities::{nodes_duties, synced_ops};
use crate::db::{
    entities::{
        nodes_duties::{ActiveModel as NodesDutiesModels, Entity as NodesDuties},
        synced_ops::{ActiveModel as SyncedOpsModel, Entity as SyncedOps},
    },
    sqlite::get_db_connection,
};
use crate::{err, errors::thot_errors::ThothErrors};

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


pub struct SqlNodesDuties {}
impl SQLiteDBTraits<NodesDuties, NodesDutiesModels> for SqlNodesDuties {
    fn find_by_id(id: String) -> Option<nodes_duties::Model> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                match NodesDuties::find_by_id(id).one(db).await {
                    Ok(Some(duty)) => Some(duty),
                    Ok(None) => None,
                    Err(e) => {
                        err!(
                            "Failed to find node duty by id due to: {}",
                            ThothErrors::from(e)
                        );
                        None
                    }
                }
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
            tokio::runtime::Handle::current().block_on(async {
                let db = get_db_connection().await;
                match NodesDuties::find()
                    .filter(nodes_duties::Column::OpId.eq(op_id))
                    .all(db)
                    .await
                {
                    Ok(duties) => duties,
                    Err(e) => {
                        err!(
                            "Failed to find node duties by op_id due to: {}",
                            ThothErrors::from(e)
                        );
                        Vec::new()
                    }
                }
            })
        })
    }
}

pub struct SqlSyncedOps {}
impl SQLiteDBTraits<SyncedOps, SyncedOpsModel> for SqlSyncedOps {
    fn find_by_id(id: String) -> Option<<SyncedOps as EntityTrait>::Model> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let db = get_db_connection().await;
                match SyncedOps::find()
                    .filter(synced_ops::Column::SyncedId.eq(id))
                    .one(db)
                    .await
                {
                    Ok(Some(op)) => Some(op),
                    Ok(None) => None,
                    Err(e) => {
                        err!(
                            "Failed to find operation by id due to: {}",
                            ThothErrors::from(e)
                        );
                        None
                    }
                }
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
                match query.one(db).await {
                    Ok(Some(model)) => Some(model),
                    Ok(None) => None,
                    Err(e) => {
                        err!(
                            "Failed to find synced operation by dates due to: {}",
                            ThothErrors::from(e)
                        );
                        None
                    }
                }
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
