mod signal;
mod runtime;
use runtime::{spawn, block_on};
use std::time::Duration;
use async_std::task;
use async_channel;

async fn do_something(){
    task::sleep(Duration::from_secs(5)).await;
}

async fn demo1(tx: async_channel::Sender<()>) {
    println!("demo1.");
    do_something().await;
    println!("waken demo1");
    let _ = tx.send(());
}

async fn demo2(tx: async_channel::Sender<()>) {
    println!("demo2.");
    do_something().await;
    println!("waken demo2");
    let _ = tx.send(());
}


async fn demo() {
    let (tx1, rx1) = async_channel::bounded::<()>(1);
    let (tx2, rx2) = async_channel::bounded::<()>(1);
    spawn(demo1(tx1));
    spawn(demo2(tx2));
    let _ = rx1.recv().await;
    let _ = rx2.recv().await;
}

// async fn demo(){
//     let (tx, rx) = async_channel::bounded::<()>(1);
//     spawn(demo2(tx));
//     println!("Hello World!");
//     let _ = rx.recv().await;
// }

// async fn demo2( tx: async_channel::Sender<()>) {
//     // task::sleep(Duration::from_secs(5)).await;
//     println!("waken demo2");
//     let _ = tx.send(());
// }


// #[async_std::main]
fn main() {
    block_on(demo());
}