use std::{
    sync::mpsc::{Receiver, Sender, channel},
};

use crate::stage::{Skeleton, Stage, StageError, Task};

pub struct Workflow<TInput, TOutput, TSkeleton: Skeleton<TInput, TOutput>> {
    terminated: bool,
    parent_stage: Stage<TInput, TOutput, TSkeleton>,
    collect_receiver: Receiver<TOutput>,
}

struct Collector<TInput> {
    sender: Sender<TInput>,
}

impl<TInput> Skeleton<TInput, TInput> for Collector<TInput> {
    fn process(&self, task: Task<TInput>) -> Result<(), StageError> {
        match task {
            Task::Value(val) => self.sender.send(val).map_err(|x| StageError::SendError),
            Task::Stop => {
                drop(self.sender);
                Ok(())
            }
        }
    }

    fn has_thread(&self) -> bool {
        false
    }
}

impl<TInput, TOutput, TSkeleton: Skeleton<TInput, TOutput>>
    Workflow<TInput, TOutput, TSkeleton>
{
    pub fn new(parent_skeleton: TSkeleton) -> Workflow<TInput, TOutput, TSkeleton> {
        let parent_stage = Stage::create(parent_skeleton);
        let (collect_sender, collect_receiver) = channel();
        let collect_stage = Stage::create(Collector { sender: collect_sender });
       
        Workflow {
            terminated: false,
            parent_stage,
            collect_receiver,
        }
    }

    pub fn end_and_collect(&mut self) -> Vec<TOutput> {
        self.terminated = true;
        self.parent_stage.send(Task::Stop);
        self.collect_receiver.iter().collect()
    }

    pub fn post(&self, task: TInput) -> Result<(), StageError> {
        return self.parent_stage.send(Task::Value(task));
    }
}
