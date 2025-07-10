use crate::{
    db::{
        configs::config::get_config,
        entities::{
            operations::Entity as Operation,
            steps::{ActiveModel as StepModel, Entity as Step},
        },
    },
    debug, err, info,
};
use sea_orm::{
    ConnectionTrait, Database, DatabaseConnection, DbBackend, DbConn, Schema,
    sea_query::TableCreateStatement,
};
use tokio::sync::OnceCell;

static CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db_connection() -> &'static DatabaseConnection {
    CONNECTION
        .get_or_try_init(|| async {
            let config = get_config();

            let database_url = format!(
                "sqlite:///{}/{}.db?mode={}",
                config.path, config.db_name, config.db_mod
            );
            debug!("database url: {}", database_url);

            Database::connect(database_url).await
        })
        .await
        .unwrap()
}

pub async fn setup_db() {
    let db: &DbConn = get_db_connection().await;

    let schema = Schema::new(DbBackend::Sqlite);

    let stmt: TableCreateStatement = schema
        .create_table_from_entity(Step)
        .if_not_exists()
        .to_owned();

    match db.execute(db.get_database_backend().build(&stmt)).await {
        Ok(_) => info!("Successfull creation of Step table."),
        Err(e) => {
            err!("Creating Step table: {}", e;panic=true);
        }
    }

    let stmt: TableCreateStatement = schema
        .create_table_from_entity(Operation)
        .if_not_exists()
        .to_owned();
    match db.execute(db.get_database_backend().build(&stmt)).await {
        Ok(_) => info!("Successfull creation of Operation table."),
        Err(e) => {
            err!("Creating Operation table: {}", e;panic=true);
        }
    }
}
