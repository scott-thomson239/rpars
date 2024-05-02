use std::sync::mpsc::Sender;

use crate::node::Node;

pub trait RPar<TInput: Send, TOutput: Send> {
    fn create_nodes(self, next_sender: Sender<TOutput>) -> (Vec<Box<dyn Node>>, Sender<TInput>);
}

pub enum Task<T: Send> {
    Value(T),
    Stop,
}
