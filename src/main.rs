use thoth::connections::channels_node_info::NodeInfoTrait;
use thoth::connections::connect::GossibConnection;
use thoth::db::sqlite::setup_db;
use thoth::err;
use thoth::errors::thot_errors::ThothErrors;
use thoth::grpc::grpc_server;
use thoth::logger::logger::LoggerWritter;
use thoth::operations::planner::organizer::Planner;
use thoth::router::post_offices::back_office::start_back_office;
use thoth::structs::structs::NodeInfo;
use tokio::spawn;
use uuid::Uuid;
#[tokio::main]
async fn main() -> Result<(), ThothErrors> {
    LoggerWritter::start().await;
    setup_db().await;
    start_back_office();
    let x: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![4.0, 5.0], vec![7.0, 8.0]];
    let y: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![4.0, 5.0]];

    spawn(async {
        let conn_res = GossibConnection::p2pconnect().await;
        if let Err(e) = conn_res {
            err!("Error Establishing Connection: {:?}", e;panic=true);
        };
    });

    use std::thread;
    use std::time::Duration;
    thread::sleep(Duration::from_secs(5));
    NodeInfo::request_other_nodes_info();
    thread::sleep(Duration::from_secs(5));
    let plan = Planner::new(Uuid::new_v4().to_string());
    let res = plan.plan_matrix_naive_multiply(x, y);
    grpc_server::start_server().await?;
    Ok(())
}
