use std::marker::PhantomData;

use crate::stage::Skeleton;

pub struct Workflow<TInput, TOutput, TSkeleton: Skeleton<TInput, TOutput>> {
    terminated: bool,
    parent_skeleton: TSkeleton,
    _marker: std::marker::PhantomData<(TInput, TOutput)>
}

impl<TInput, TOutput, TSkeleton: Skeleton<TInput, TOutput>> Workflow<TInput, TOutput, TSkeleton> {
    pub fn new(parent_skeleton: TSkeleton) -> Workflow<TInput, TOutput, TSkeleton> {
        Workflow {
            terminated: false,
            parent_skeleton,
            _marker: PhantomData
        }
    }

    pub fn end_and_wait(&mut self) {
        self.terminated = true;
    }

    pub fn post(&self, task: TInput) -> Result<(), PostError> {
        return Ok(())
    }

    pub fn collect(&self) -> Vec<TOutput> {
        return vec![]
    }
}

pub enum PostError {
    WorkflowTerminated,
}
