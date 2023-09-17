use std::{marker::PhantomData, sync::{Arc, Mutex}};

use crate::{
    handler::Handler,
    rpar::{RPar, Task, RParTemplate},
};

pub struct Seq<TInput: Send, TOutput: Send> {
    seq_fun: fn(TInput) -> TOutput,
    next: Arc<Mutex<Handler<TOutput>>>,
}

impl<TInput: 'static + Send, TOutput: 'static + Send> Seq<TInput, TOutput> {
    fn new<F: FnOnce(Handler<TOutput>) -> Seq<TInput, TOutput>>(
        seq_fun: fn(TInput) -> TOutput,
    ) -> RParTemplate<TInput, TOutput, Seq<TInput, TOutput>> {
        RParTemplate {
            f: Arc::new(Box::new(move |next_handler| Seq {
                seq_fun,
                next: next_handler,
            })),
            _marker: PhantomData
        }
    }
}

impl<TInput: 'static + Send, TOutput: 'static + Send> RPar<TInput> for Seq<TInput, TOutput> {
    fn process(&mut self, task: Task<TInput>) -> Result<(), ()> {
        let mut handler = self.next.lock().unwrap();
        match task {
            Task::Value(val) => handler.send(Task::Value((self.seq_fun)(val))),
            Task::Stop => handler.send(Task::Stop),
        }
    }
}
