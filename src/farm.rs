use std::{
    marker::PhantomData,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::{node::Node, rpar::RPar};

pub struct Farm<TInput: Send + 'static, TOutput: Send + 'static, TRPar: RPar<TInput, TOutput>> {
    replicas: usize,
    rpar: TRPar,
    _marker: PhantomData<(TInput, TOutput)>,
}

impl<TInput: Send + 'static, TOutput: Send + 'static, TRPar: RPar<TInput, TOutput> + Clone>
    RPar<TInput, TOutput> for Farm<TInput, TOutput, TRPar>
{
    fn create_nodes(self, next_sender: Sender<TOutput>) -> (Vec<Box<dyn Node>>, Sender<TInput>) {
        let (emitter_sender, emitter_receiver) = channel();
        let mut replicas = vec![];
        replicas.resize(self.replicas, self.rpar);
        let nodes_iter = replicas
            .into_iter()
            .map(|par| par.create_nodes(next_sender.clone()));
        let (stage_nodes, replica_senders): (Vec<_>, Vec<_>) = nodes_iter.unzip();
        let mut all_nodes: Vec<Box<dyn Node>> = stage_nodes.into_iter().flatten().collect();
        let emitter_node = Box::new(FarmEmitterNode {
            receiver: emitter_receiver,
            cur_index: 0,
            replica_senders,
        });
        all_nodes.push(emitter_node);
        (all_nodes, emitter_sender)
    }
}

pub struct FarmEmitterNode<TInput: Send> {
    receiver: Receiver<TInput>,
    cur_index: usize,
    replica_senders: Vec<Sender<TInput>>,
}

impl<TInput: Send> Node for FarmEmitterNode<TInput> {
    fn try_process_next(&mut self) -> Result<(), ()> {
        let next = self.receiver.try_recv().map_err(|_| ())?;
        self.replica_senders[self.cur_index]
            .send(next)
            .map_err(|_| ())?;
        self.cur_index = (self.cur_index + 1) % self.replica_senders.len();
        Ok(())
    }
}
