use crate::rpar::{RPar, Task};
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

pub struct Handler<TInput: Send> {
    terminated: bool,
    sender: Sender<Task<TInput>>,
    join_handle: JoinHandle<()>,
}

impl<TInput: Send + 'static> Handler<TInput> {
    pub fn new<TRPar: 'static + Send + RPar<TInput>>(rpar: TRPar) -> Handler<TInput> {
        let (sender, receiver) = channel();
        Handler {
            terminated: false,
            sender,
            join_handle: thread::spawn(move || Handler::thread_loop(rpar, receiver)),
        }
    }

    pub fn send(&mut self, task: Task<TInput>) -> Result<(), ()> {
        match task {
            Task::Stop => {
                self.terminated = true;
            }
            _ => (),
        }
        match self {
            Handler {
                terminated: false,
                sender,
                join_handle: _,
            } => sender.send(task).map_err(|_| ()),
            _ => Err(()),
        }
    }

    fn thread_loop<TRPar: RPar<TInput>>(mut rpar: TRPar, receiver: Receiver<Task<TInput>>) {
        for task in receiver.iter() {
            rpar.process(task);
        }
    }
}
