use thoth::connections::connect::GossibConnection;
#[tokio::main]
async fn main() {
    let conn_res = GossibConnection::p2pconnect().await;
    if conn_res.is_err() {
        println!("Error Establishing Connection: {:?}", conn_res.err());
    }
}
