use std::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    node::Node,
    rpar::{RPar, Task},
    threadpool::ThreadPool,
};

pub struct Workflow<TInput: Send, TOutput: Send> {
    sender: Sender<Task<TInput>>,
    res_receiver: Receiver<Task<TOutput>>,
    thread_pool: ThreadPool,
}

impl<TInput: Send, TOutput: Send> Workflow<TInput, TOutput> {
    fn new<TRPar: RPar<TInput, TOutput>>(
        rpar: TRPar,
        max_parallel: usize,
    ) -> Workflow<TInput, TOutput> {
        let (res_sender, res_receiver) = channel();
        let (nodes, sender) = rpar.create_nodes(res_sender);
        let thread_pool = ThreadPool::start(nodes, max_parallel);
        Workflow {
            sender,
            res_receiver,
            thread_pool,
        }
    }

    fn send(&self, val: TInput) -> Result<(), ()> {
        self.sender.send(Task::Value(val)).map_err(|_| ())
    }

    fn end_and_collect(self) -> Result<Vec<TOutput>, ()> {
        self.sender.send(Task::Stop).map_err(|_| ())?;
        self.thread_pool.wait_all().map_err(|_| ())?;
        Ok(self
            .res_receiver
            .iter()
            .filter_map(|task| match task {
                Task::Value(v) => Some(v),
                Task::Stop => None,
            })
            .collect())
    }
}
