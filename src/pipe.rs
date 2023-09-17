use crate::{rpar::{Task, RPar}, handler::Handler};

pub struct Pipe<TInput: Send> {
    first: Handler<TInput>,
}

impl<TInput: Send> Pipe<TInput> {
    //todo
}

impl<TInput: 'static + Send> RPar<TInput> for Pipe<TInput> {
   fn process(&mut self, task: Task<TInput>) -> Result<(), ()> {
      self.first.send(task)
   }
}
