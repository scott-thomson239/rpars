use std::sync::mpsc::Sender;

use crate::node::Node;

pub trait RPar<TInput: Send, TOutput: Send> {
    fn create_nodes(
        self,
        next_sender: Sender<Task<TOutput>>,
    ) -> (Vec<Box<dyn Node>>, Sender<Task<TInput>>);
}

#[derive(Debug, PartialEq, Eq)]
pub enum Task<T: Send> {
    Value(T),
    Stop,
}
