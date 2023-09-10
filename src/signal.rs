use std::sync::{Arc, Mutex, Condvar};
use std::task::Wake;
thread_local!(static SIGNAL: Signal = Signal::new(););

pub struct Signal {
    state: Mutex<State>,
    cond: Condvar,
}

unsafe impl Send for Signal {}
unsafe impl Sync for Signal {}

enum State {
    Empty,
    Waiting,
    Notified,
}

impl Signal{
    pub fn new() -> Self{
        Signal { state: Mutex::new(State::Empty), cond: (Condvar::new()) }
    }
    pub fn wait(&self){
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Notified => *state = State::Empty,
            State::Waiting => {
                panic!("multipole wait");
            }
            State::Empty => {
                *state = State::Waiting;
                while let State::Waiting = *state {
                    state = self.cond.wait(state).unwrap();
                }
            }
        }
    }
    pub fn notify(&self){
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Notified => {}
            State::Empty => *state = State::Notified,
            State::Waiting => {
                *state = State::Empty;
                self.cond.notify_one();
            }
        }

    }
}

impl Wake for Signal{
    fn wake(self: Arc<Self>){
        // println!("{:?}",std::thread::current().id());
        self.notify();
        SIGNAL.with(|signal|{
            signal.notify();
        });
    }
}