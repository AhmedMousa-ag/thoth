use std::sync::{
    Arc,
    mpsc::{self, Sender},
};
use tokio::spawn;
pub enum MatrixOperations {
    MULTIPLY,
    DIVIDE,
}
#[derive(Debug)]
struct IDXRes {
    i: usize,
    j: usize,
    res: f64,
}

pub fn multiply(x: Vec<Vec<f64>>, y: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // Naive implementation of matrix multipication.
    /*
        3*2      2*2
        [1 2]    [1 2]
        [4 5]    [4 5]
        [7 8]

        [1*1 + 2*4,   1*2 + 2*5]
        [4*1 + 5*4,   4*2 + 5*5]
        [7*1 + 8*4,   7*2 + 8*5]

        [9 12]
        [24 33]
        [39 54]
    */
    let (res_sender, res_reciever) = mpsc::channel();
    let res_sender: Arc<Sender<IDXRes>> = Arc::new(res_sender);

    let xarc = Arc::new(x);
    let yarc = Arc::new(y);
    spawn(async move {
        // Will do a naive approach as strassen multipication can accumulates errors due to precisions.
        for (i, _) in xarc.iter().enumerate() {
            for (j, col_num) in yarc.iter().enumerate() {
                for (k, _) in col_num.iter().enumerate() {
                    let xarc = Arc::clone(&xarc);
                    let yarc = Arc::clone(&yarc);
                    let res_sender = Arc::clone(&res_sender);
                    spawn(async move {
                        let res = xarc[i][k] * yarc[k][j];
                        //TODO log error. and Handle it.
                        let _ = res_sender.send(IDXRes { i, j, res });
                    });
                }
            }
        }
    });

    let mut result: Vec<Vec<f64>> = Vec::new();
    let mut icounter = 0;
    //TODO raise an error when the matrix dimensions can't be multiplied.
    for msg in res_reciever {
        while icounter <= msg.i {
            result.push(vec![0.0]);
            icounter += 1;
        }
        while result[msg.i].get(msg.j).is_none() {
            result[msg.i].push(0.0);
        }

        result[msg.i][msg.j] += msg.res;
    }
    result
}
