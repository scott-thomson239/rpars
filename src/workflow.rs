use std::sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex};

use crate::{
    handler::Handler,
    rpar::{RPar, RParTemplate, Task},
};

pub struct Workflow<TInput: Send, TOutput: Send> {
    terminated: bool,
    handler: Handler<TInput>,
    collect_receiver: Receiver<TOutput>,
}

impl<TInput: 'static + Send, TOutput: 'static + Send> Workflow<TInput, TOutput> {
    fn new<TRPar: 'static + Send + RPar<TInput>, F: FnOnce(Handler<TOutput>) -> TRPar>(
        rpar_template: RParTemplate<TInput, TOutput, TRPar>,
    ) -> Workflow<TInput, TOutput> {
        let (sender, collect_receiver) = channel();
        let collector = Handler::new(Collector{ sender });
        let handler = rpar_template.apply(Arc::new(Mutex::new(collector)));
        Workflow {
            terminated: false,
            handler: Handler::new(handler),
            collect_receiver
        }
    }

    fn send(&mut self, val: TInput) -> Result<(), ()> {
        self.handler.send(Task::Value(val))
    }

    fn end_and_collect(&mut self) -> Vec<TOutput> {
        self.terminated = true;
        self.handler.send(Task::Stop);
        self.collect_receiver.iter().collect()
    }
}

struct Collector<TInput: Send> {
    sender: Sender<TInput>,
}

impl<TInput: Send> RPar<TInput> for Collector<TInput> {
    fn process(&mut self, task: Task<TInput>) -> Result<(), ()> {
        match task {
            Task::Value(val) => self.sender.send(val).map_err(|_| ()),
            Task::Stop => Ok(()),
        }
    }
}
