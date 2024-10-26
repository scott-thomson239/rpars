use crate::node::Node;
use itertools::Itertools;
use std::thread;
use std::thread::Result;

pub enum WaitError {
    JoinError,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn start(nodes: Vec<Box<dyn Node>>, max_workers: usize) -> ThreadPool {
        let node_chunk_size = nodes.len() / max_workers;
        let workers = nodes
            .into_iter()
            .chunks(node_chunk_size)
            .into_iter()
            .map(|node_chunk| Worker::new(node_chunk.collect_vec()))
            .collect();
        ThreadPool { workers }
    }

    pub fn wait_all(self) -> Result<()> {
        self.workers.into_iter().try_for_each(|a| a.wait())?;
        Ok(())
    }
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(mut nodes: Vec<Box<dyn Node>>) -> Worker {
        let thread = thread::spawn(move || {
            for idx in (0..nodes.len()).cycle() {
                let node = &mut nodes[idx];
                let res = node.try_process_next();
                if res.is_err() {
                    break;
                }
            }
        });

        Worker { thread }
    }

    fn wait(self) -> Result<()> {
        self.thread.join()
    }
}
