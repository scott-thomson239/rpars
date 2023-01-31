use std::sync::mpsc::{channel, Receiver, SendError, Sender};

pub trait Skeleton<Input, Output> {
    fn process(&self, input: Task<Input>) -> Result<(), SendError<Output>>;
}

pub enum Task<T> {
    Value(T),
    Stop,
}

pub struct Stage<TInput, TOutput, TSkeleton: Skeleton<TInput, TOutput>> {
    input_channel: Receiver<Task<TInput>>,
    output_channel: Sender<Task<TOutput>>,
    SkeletonMaker: TSkeleton,
}
