use std::{
    sync::mpsc::{channel, Receiver, SendError, Sender},
    thread::{self, JoinHandle},
};

pub trait Skeleton<TInput, TOutput> {
    fn process(&self, task: Task<TInput>) -> Result<(), StageError>;
    fn has_thread(&self) -> bool;
}

pub enum Task<T> {
    Value(T),
    Stop,
}

pub enum StageError {
    SendError,
    JoinError,
}

pub struct Stage<TInput, TOutput, TSkeleton: Skeleton<TInput, TOutput>> {
    connection: Option<SkeletonConn<Task<TInput>>>,
    skeleton: TSkeleton,
    _marker: std::marker::PhantomData<(TInput, TOutput)>,
}

struct SkeletonConn<TInput> {
    thread: JoinHandle<()>,
    sender: Sender<TInput>,
}

impl<TInput, TOutput, TSkeleton: Skeleton<TInput, TOutput>> Stage<TInput, TOutput, TSkeleton> {
    pub fn create(skeleton: TSkeleton) -> Stage<TInput, TOutput, TSkeleton> {
        let connection = if skeleton.has_thread() {
            let (sender, receiver) = channel();
            Some(SkeletonConn {
                thread: thread::spawn(move || Stage::thread_loop(&skeleton, receiver)),//
                sender,
            })
        } else {
            None
        };
        Stage {
            connection,
            skeleton,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn send(&self, task: Task<TInput>) -> Result<(), StageError> {
        self.skeleton
            .process(task)
            .map_err(|x| StageError::SendError)?;
        match task {
            Task::Value(v) => Ok(()),
            Task::Stop => self
            .connection
            .map(|conn| conn.thread.join())
            .transpose()
            .map(|x| ())
            .map_err(|x| StageError::JoinError),
        }
    }

    fn thread_loop(skeleton: &TSkeleton, receiver: Receiver<Task<TInput>>) -> () {
        for val in receiver.iter() {
            skeleton.process(val); //handle error
        }
    }
}
