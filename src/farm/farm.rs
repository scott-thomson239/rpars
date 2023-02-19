use crate::stage::{Skeleton, Stage, StageError, Task};

pub struct Farm<TInput, TOutput, TSkeleton: Skeleton<TInput, TOutput>> {
    stages: Vec<Stage<TInput, TOutput, TSkeleton>>,
}

impl<TInput, TOutput, TSkeleton: Skeleton<TInput, TOutput>> Skeleton<TInput, TOutput>
    for Farm<TInput, TOutput, TSkeleton>
{
    fn process(&self, task: Task<TInput>) -> Result<(), StageError> {
        self.stages[0].send(task)
    }

    fn has_thread(&self) -> bool {
        false
    }
}
