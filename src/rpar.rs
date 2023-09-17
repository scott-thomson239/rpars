use std::{marker::PhantomData, sync::{Arc, Mutex}};

use crate::handler::Handler;

pub struct RParTemplate<TInput: Send, TOutput: Send, TRPar: RPar<TInput>> {
   pub f: Arc<Box<dyn Fn(Arc<Mutex<Handler<TOutput>>>) -> TRPar>>,
   pub _marker: PhantomData<TInput>
}

impl<TInput: Send, TOutput: Send, TRPar: RPar<TInput>> Clone for RParTemplate<TInput, TOutput, TRPar> {
   fn clone(&self) -> Self {
      Self {
         f: self.f.clone(),
         _marker: self._marker.clone()
      }
   }
}

impl<TInput: Send, TOutput: Send, TRPar: RPar<TInput>> RParTemplate<TInput, TOutput, TRPar> {
   pub fn apply(&self, next: Arc<Mutex<Handler<TOutput>>>) -> TRPar {
      (*self.f)(next)
   }
}

pub trait RPar<TInput: Send> {
   fn process(&mut self, task: Task<TInput>) -> Result<(), ()>;
}

pub enum Task<T: Send> {
    Value(T),
    Stop,
}
