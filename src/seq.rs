use std::sync::mpsc::{channel, Receiver, Sender};

use crate::{node::Node, rpar::RPar};

pub struct Seq<TInput: Send + 'static, TOutput: Send + 'static> {
    f: fn(TInput) -> TOutput,
}

impl<TInput: Send + 'static, TOutput: Send + 'static> RPar<TInput, TOutput>
    for Seq<TInput, TOutput>
{
    fn create_nodes(self, next_sender: Sender<TOutput>) -> (Vec<Box<dyn Node>>, Sender<TInput>) {
        let (sender, receiver) = channel();
        let seq_node = Box::new(SeqNode {
            f: self.f,
            receiver,
            sender: next_sender,
        });
        (vec![seq_node], sender)
    }
}

pub struct SeqNode<TInput: Send, TOutput: Send> {
    f: fn(TInput) -> TOutput,
    receiver: Receiver<TInput>,
    sender: Sender<TOutput>,
}

impl<TInput: Send, TOutput: Send> Node for SeqNode<TInput, TOutput> {
    fn try_process_next(&mut self) -> Result<(), ()> {
        let next = self.receiver.try_recv().map_err(|_| ())?;
        let output = (self.f)(next);
        self.sender.send(output).map_err(|_| ())
    }
}
