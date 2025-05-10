use thoth::connections::connect::p2pconnect;

#[tokio::main]
async fn main() {
    let conn_res = p2pconnect().await;
    if conn_res.is_err() {
        println!("Error Establishing Connection: {:?}", conn_res.err());
    }
}
