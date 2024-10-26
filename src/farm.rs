use std::{
    marker::PhantomData,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::{
    node::{Node, TryProcessError},
    rpar::{RPar, Task},
};

pub struct Farm<TInput: Send + 'static, TOutput: Send + 'static, TRPar: RPar<TInput, TOutput>> {
    replicas: usize,
    rpar: TRPar,
    _marker: PhantomData<(TInput, TOutput)>,
}

impl<TInput: Send + 'static, TOutput: Send + 'static, TRPar: RPar<TInput, TOutput> + Clone>
    Farm<TInput, TOutput, TRPar>
{
    pub fn new(replicas: usize, rpar: TRPar) -> Farm<TInput, TOutput, TRPar> {
        Farm {
            replicas,
            rpar,
            _marker: PhantomData,
        }
    }
}

impl<TInput: Send + 'static, TOutput: Send + 'static, TRPar: RPar<TInput, TOutput> + Clone>
    RPar<TInput, TOutput> for Farm<TInput, TOutput, TRPar>
{
    fn create_nodes(
        self,
        next_sender: Sender<Task<TOutput>>,
    ) -> (Vec<Box<dyn Node>>, Sender<Task<TInput>>) {
        let (emitter_sender, emitter_receiver) = channel();
        let mut replicas = vec![];
        replicas.resize(self.replicas, self.rpar);
        let nodes_iter = replicas
            .into_iter()
            .map(|par| par.create_nodes(next_sender.clone()));
        let (stage_nodes, replica_senders): (Vec<_>, Vec<_>) = nodes_iter.unzip();
        let mut all_nodes: Vec<Box<dyn Node>> = stage_nodes.into_iter().flatten().collect();
        let emitter_node = Box::new(FarmEmitterNode::new(emitter_receiver, replica_senders));
        all_nodes.push(emitter_node);
        (all_nodes, emitter_sender)
    }
}

pub struct FarmEmitterNode<TInput: Send> {
    receiver: Receiver<Task<TInput>>,
    cur_index: usize,
    replica_senders: Vec<Sender<Task<TInput>>>,
    cancelled: bool,
}

impl<TInput: Send> FarmEmitterNode<TInput> {
    fn new(
        receiver: Receiver<Task<TInput>>,
        replica_senders: Vec<Sender<Task<TInput>>>,
    ) -> FarmEmitterNode<TInput> {
        FarmEmitterNode {
            receiver,
            cur_index: 0,
            replica_senders,
            cancelled: false,
        }
    }
}

impl<TInput: Send> Node for FarmEmitterNode<TInput> {
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
                self.replica_senders[self.cur_index]
                    .send(Task::Value(v))
                    .map_err(|_| TryProcessError::ProcessError)?;
                self.cur_index = (self.cur_index + 1) % self.replica_senders.len();
                Ok(())
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
    fn input_sent_to_consecutive_nodes() {
        let (in_sender, in_receiver) = channel();
        let (replica_1_sender, replica_1_receiver) = channel();
        let (replica_2_sender, replica_2_receiver) = channel();
        let (replica_3_sender, replica_3_receiver) = channel();

        let replica_senders = vec![replica_1_sender, replica_2_sender, replica_3_sender];
        let mut farm_emitter_node: FarmEmitterNode<i32> =
            FarmEmitterNode::new(in_receiver, replica_senders);

        in_sender.send(Task::Value(1)).unwrap();
        in_sender.send(Task::Value(2)).unwrap();
        in_sender.send(Task::Value(3)).unwrap();
        let thread = thread::spawn(move || {
            let mut counter = 0;
            loop {
                let res = farm_emitter_node.try_process_next();
                match res {
                    Ok(_) => {
                        counter += 1;
                        if counter >= 3 {
                            break;
                        }
                    }
                    Err(TryProcessError::Empty) => (),
                    _ => break,
                }
            }
        });
        thread.join().unwrap();
        let res1 = replica_1_receiver.recv().unwrap();
        assert_eq!(res1, Task::Value(1));
        let res2 = replica_2_receiver.recv().unwrap();
        assert_eq!(res2, Task::Value(2));
        let res3 = replica_3_receiver.recv().unwrap();
        assert_eq!(res3, Task::Value(3));
    }
}
