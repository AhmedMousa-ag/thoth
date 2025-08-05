use thoth::connections::connect::GossibConnection;
use thoth::db::sqlite::setup_db;
use thoth::err;
use thoth::errors::thot_errors::ThothErrors;
use thoth::grpc::grpc_server;
use thoth::logger::logger::LoggerWritter;
use tokio::spawn;
#[tokio::main]
async fn main() -> Result<(), ThothErrors> {
    LoggerWritter::start().await;
    setup_db().await;
    spawn(async {
        let conn_res = GossibConnection::p2pconnect().await;
        if let Err(e) = conn_res {
            err!("Error Establishing Connection: {:?}", e;panic=true);
        };
    });
    grpc_server::start_server().await?;
    Ok(())
}
