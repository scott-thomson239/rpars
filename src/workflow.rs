use std::sync::mpsc::{channel, Receiver, Sender};

use crate::{node::Node, rpar::RPar, threadpool::ThreadPool};

pub struct Workflow<TInput: Send, TOutput: Send> {
    terminated: bool,
    sender: Sender<TInput>,
    res_receiver: Receiver<TOutput>,
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
            terminated: false,
            sender,
            res_receiver,
            thread_pool,
        }
    }

    fn send(&self, val: TInput) -> Result<(), ()> {
        self.sender.send(val).map_err(|_| ())
    }

    fn end_and_collect(&mut self) -> Vec<TOutput> {
        self.terminated = true;
        self.res_receiver.iter().collect()
    }
}
