use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

use crate::stage::{Skeleton, Stage, Task, StageError};

pub struct Pipe<TInput, TFirstOutput, TSkeleton: Skeleton<TInput, TFirstOutput>> {
    first_stage: Stage<TInput, TFirstOutput, TSkeleton>,
    _marker: std::marker::PhantomData<(TInput, TFirstOutput)>,
}

impl<TInput, TFirstOutput, TSkeleton: Skeleton<TInput, TFirstOutput>> Skeleton<TInput, TFirstOutput>
    for Pipe<TInput, TFirstOutput, TSkeleton>
{
    fn process(&self, task: Task<TInput>) -> Result<(), StageError> {
        self.first_stage.send(task).map_err(|x| StageError::SendError)
    }

    fn has_thread(&self) -> bool {
        false
    }
}
