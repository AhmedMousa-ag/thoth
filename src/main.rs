use thoth::connections::connect::GossibConnection;
use thoth::err;
use thoth::logger::logger::LoggerWritter;
#[tokio::main]
async fn main() {
    LoggerWritter::start().await;
    let conn_res = GossibConnection::p2pconnect().await;
    if conn_res.is_err() {
        err!("Error Establishing Connection: {:?}", conn_res.err());
    }
}
