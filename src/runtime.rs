use std::cell::RefCell;
use std::future::Future;
use std::sync::Arc;
use std::sync::Mutex; 
use futures::future::BoxFuture;
use std::collections::VecDeque;
use crate::signal::Signal;
use std::task::{Waker, Wake, Context, Poll};

thread_local!(static RUNNABLE: Arc<Mutex<VecDeque<Arc<Task>>>> = Arc::new(Mutex::new(VecDeque::new())));
pub fn spawn(fut: impl Future<Output = ()> + 'static + std::marker::Send) {
    let t = Arc::new(Task {
        // future: RefCell::new(fut.boxed()),
        future: RefCell::new(Box::pin(fut)),
        signal: Arc::new(Signal::new()),
    });
    // println!("{:?}!",thread::current().id());
    RUNNABLE.with(|runnable| {
        runnable.lock().unwrap().push_back(t.clone());
    });
}

struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}
unsafe impl Send for Task {}
unsafe impl Sync for Task {}
impl Wake for Task{
    fn wake(self: Arc<Self>) {
        // println!("{:?}!!",thread::current().id());
        RUNNABLE.with(|runnable| {
            runnable.lock().unwrap().push_back(self.clone());
            self.signal.notify();
        });
    }
}

pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut fut = std::pin::pin!(future);
    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone());
    let mut cx = Context::from_waker(&waker);
    // let runnable = Arc::new(Mutex::new(VecDeque::new()));
    // SIGNAL.set(&signal, ||{
    //     RUNNABLE.set(&runnable, ||{
            loop {
                // println!("in main loop");
                // if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
                //     return output;
                // }
                // match fut.as_mut().poll(&mut cx) {
                //     Poll::Ready(output) => {
                //         println!("ready");
                //         return output;
                //     }
                //     Poll::Pending => {
                //         println!("pending");
                //     }
                // }
                if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
                    return output;
                }
                while let Some(task) = RUNNABLE.with(|runnable| {runnable.lock().unwrap().pop_front()}){
                    let waker = Waker::from(task.clone());
                    let mut cx = Context::from_waker(&waker);
                    let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                    // match task.future.borrow_mut().as_mut().poll(&mut cx) {
                    //     Poll::Ready(_) => {
                    //         println!("ready");
                    //     }
                    //     Poll::Pending => {
                    //         println!("pending");
                    //         // task.signal.wait();
                    //     }
                    // }
                }
                signal.wait()
            }
        // })
    // })

}