use thoth::connections::connect::GossibConnection;
use thoth::err;
use thoth::grpc::grpc_server;
use thoth::logger::logger::LoggerWritter;
use tokio::spawn;
#[tokio::main]
async fn main() {
    LoggerWritter::start().await;
    spawn(async { grpc_server::start_server().await });
    let conn_res = GossibConnection::p2pconnect().await;
    if conn_res.is_err() {
        err!("Error Establishing Connection: {:?}", conn_res.err());
    }
}
