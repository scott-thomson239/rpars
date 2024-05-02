use crate::node::Node;
use std::thread::{self, Thread};

pub struct ThreadPool {
    nodes: Vec<Box<dyn Node>>,
    workers: Vec<Worker>,
    threads: Vec<Thread>,
}

impl ThreadPool {
    pub fn start(nodes: Vec<Box<dyn Node>>, max_workers: usize) -> ThreadPool {
        todo!()
    }

    pub fn cancel() -> Result<(), ()> {
        Ok(())
    }
}

#[derive(Clone)]
struct Worker {
    cancelled: bool,
}

impl Worker {
    fn new() -> Worker {
        Worker { cancelled: false }
    }

    fn run_until_cancelled(&mut self, nodes: &mut Vec<Box<dyn Node>>) -> Result<(), ()> {
        for idx in (0..nodes.len()).cycle() {
            let node = &mut nodes[idx];
            node.try_process_next()?; // TODO: block on first receiver instead of busy wait if it wont cause deadlocks
            if self.cancelled {
                break;
            }
        }
        Ok(())
    }

    fn cancel(&mut self) -> bool {
        match self.cancelled {
            true => false,
            false => {
                self.cancelled = true;
                true
            }
        }
    }
}
