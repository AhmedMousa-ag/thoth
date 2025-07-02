use thoth::connections::connect::GossibConnection;
use thoth::grpc::grpc_server;
use thoth::logger::logger::LoggerWritter;
use thoth::operations::planner::organizer::Planner;
use thoth::router::post_offices::back_office::start_back_office;
use thoth::{debug, err};
use tokio::spawn;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    LoggerWritter::start().await;
    start_back_office();
    let x: Vec<f64> = vec![1.0, 2.0, 3.5, 7.5]; //, vec![4.0, 5.0]];
    let y: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![4.0, 5.0]];

    spawn(async {
        let conn_res = GossibConnection::p2pconnect().await;
        if conn_res.is_err() {
            let err = conn_res.err();
            err!("Error Establishing Connection: {:?}", err);
            panic!("Error Establishing Connection: {:?}", err);
        };
    });
    use std::thread;
    use std::time::Duration;
    thread::sleep(Duration::from_secs(5));
    let plan = Planner::new();
    debug!("Created planner");
    let res = plan.plan_average(x);
    debug!("{:?}", res);
    grpc_server::start_server().await?;
    Ok(())
}
