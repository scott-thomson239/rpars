use std::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    node::{Node, TryProcessError},
    rpar::{RPar, Task},
};

pub struct Seq<TInput: Send + 'static, TOutput: Send + 'static> {
    f: fn(TInput) -> TOutput,
}

impl<TInput: Send + 'static, TOutput: Send + 'static> RPar<TInput, TOutput>
    for Seq<TInput, TOutput>
{
    fn create_nodes(
        self,
        next_sender: Sender<Task<TOutput>>,
    ) -> (Vec<Box<dyn Node>>, Sender<Task<TInput>>) {
        let (sender, receiver) = channel();
        let seq_node = Box::new(SeqNode {
            f: self.f,
            receiver,
            sender: next_sender,
            cancelled: false,
        });
        (vec![seq_node], sender)
    }
}

pub struct SeqNode<TInput: Send, TOutput: Send> {
    f: fn(TInput) -> TOutput,
    receiver: Receiver<Task<TInput>>,
    sender: Sender<Task<TOutput>>,
    cancelled: bool,
}

impl<TInput: Send, TOutput: Send> Node for SeqNode<TInput, TOutput> {
    fn try_process_next(&mut self) -> Result<(), TryProcessError> {
        let next = self
            .receiver
            .try_recv()
            .map_err(|_| TryProcessError::ProcessError)?;
        match next {
            Task::Value(v) => {
                let output = (self.f)(v);
                self.sender
                    .send(Task::Value(output))
                    .map_err(|_| TryProcessError::ProcessError)
            }
            Task::Stop => {
                self.cancelled = true;
                Err(TryProcessError::NodeCancelled)
            }
        }
    }
}
