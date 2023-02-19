use crate::stage::{Skeleton, Stage, Task, StageError};

pub struct Seq<TInput, TOutput, TNextOutput, TSkeleton: Skeleton<TOutput, TNextOutput>> {
    next_stage: Stage<TOutput, TNextOutput, TSkeleton>,
    seq_fun: fn(TInput) -> TOutput,
}

impl<TInput, TOutput, TNextOutput, TSkeleton: Skeleton<TOutput, TNextOutput>>
    Skeleton<TInput, TOutput> for Seq<TInput, TOutput, TNextOutput, TSkeleton>
{
    fn process(&self, task: Task<TInput>) -> Result<(), StageError> {
        match task {
            Task::Value(val) => self.next_stage.send(Task::Value((self.seq_fun)(val))),
            Task::Stop => self.next_stage.send(Task::Stop)
        }
    }

    fn has_thread(&self) -> bool {
        true
    }
}
