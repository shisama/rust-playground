extern crate tokio_threadpool;
extern crate futures;

use tokio_threadpool::ThreadPool;
use futures::{Future, lazy};
use futures::sync::oneshot;

pub fn main() {
    let pool1 = ThreadPool::new();
    let (tx1, rx1) = oneshot::channel();

    pool1.spawn(lazy(|| {
        println!("Running on the pool1");
        tx1.send("complete").map_err(|e| println!("send error, {}", e))
    }));


    let pool2 = ThreadPool::new();
    let (tx2, rx2) = oneshot::channel();

    pool2.spawn(lazy(|| {
        println!("Running on the pool2");
        tx2.send("complete").map_err(|e| println!("send error, {}", e))
    }));

    println!("Result1: {:?}", rx1.wait());
    println!("Result2: {:?}", rx2.wait());
    pool1.shutdown().wait().unwrap();
    pool2.shutdown().wait().unwrap();
}
