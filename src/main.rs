use thoth::connections::channels_node_info::NodeInfoTrait;
use thoth::connections::connect::GossibConnection;
use thoth::db::sqlite::setup_db;
use thoth::grpc::grpc_server;
use thoth::logger::logger::LoggerWritter;
use thoth::operations::planner::organizer::Planner;
use thoth::router::post_offices::back_office::start_back_office;
use thoth::structs::structs::NodeInfo;
use thoth::{debug, err};
use tokio::spawn;
use uuid::Uuid;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    LoggerWritter::start().await;
    setup_db().await;
    start_back_office();
    let x: Vec<Vec<Box<f64>>> = vec![
        vec![Box::new(1.0), Box::new(2.0)],
        vec![Box::new(4.0), Box::new(5.0)],
        vec![Box::new(7.0), Box::new(8.0)],
    ];
    let y: Vec<Vec<Box<f64>>> = vec![
        vec![Box::new(1.0), Box::new(2.0)],
        vec![Box::new(4.0), Box::new(5.0)],
    ];

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
    NodeInfo::request_other_nodes_info();
    thread::sleep(Duration::from_secs(5));
    let plan = Planner::new();
    debug!("Created planner");
    let res = plan.plan_matrix_naive_multiply(x, y, Uuid::new_v4().to_string());
    debug!("{:?}", res);
    grpc_server::start_server().await?;
    Ok(())
}
