use std::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    node::{Node, TryProcessError},
    rpar::{RPar, Task},
};

pub struct Seq<TInput: Send + 'static, TOutput: Send + 'static> {
    f: fn(TInput) -> TOutput,
}

impl<TInput: Send + 'static, TOutput: Send + 'static> Seq<TInput, TOutput> {
    pub fn new(f: fn(TInput) -> TOutput) -> Seq<TInput, TOutput> {
        Seq { f }
    }
}

impl<TInput: Send + 'static, TOutput: Send + 'static> RPar<TInput, TOutput>
    for Seq<TInput, TOutput>
{
    fn create_nodes(
        self,
        next_sender: Sender<Task<TOutput>>,
    ) -> (Vec<Box<dyn Node>>, Sender<Task<TInput>>) {
        let (sender, receiver) = channel();
        let seq_node = Box::new(SeqNode::new(self.f, receiver, next_sender));
        (vec![seq_node], sender)
    }
}

pub struct SeqNode<TInput: Send, TOutput: Send> {
    f: fn(TInput) -> TOutput,
    receiver: Receiver<Task<TInput>>,
    sender: Sender<Task<TOutput>>,
    cancelled: bool,
}

impl<TInput: Send, TOutput: Send> SeqNode<TInput, TOutput> {
    fn new(
        f: fn(TInput) -> TOutput,
        receiver: Receiver<Task<TInput>>,
        sender: Sender<Task<TOutput>>,
    ) -> SeqNode<TInput, TOutput> {
        SeqNode {
            f,
            receiver,
            sender,
            cancelled: false,
        }
    }
}

impl<TInput: Send, TOutput: Send> Node for SeqNode<TInput, TOutput> {
    fn try_process_next(&mut self) -> Result<(), TryProcessError> {
        if self.cancelled {
            return Err(TryProcessError::NodeCancelled);
        }

        let next = self
            .receiver
            .try_recv()
            .map_err(|_| TryProcessError::Empty)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn input_sent_to_output_receiver() {
        let f = |x: i32| (x + 1).to_string();
        let (in_sender, in_receiver) = channel();
        let (out_sender, out_receiver) = channel();

        let mut seq_node = SeqNode::new(f, in_receiver, out_sender);
        in_sender.send(Task::Value(10)).unwrap();
        let thread = thread::spawn(move || loop {
            let res = seq_node.try_process_next();
            if res != Err(TryProcessError::Empty) {
                break;
            }
        });
        thread.join().unwrap();
        let res = out_receiver.recv().unwrap();
        assert_eq!(res, Task::Value(String::from("11")));
    }
}
