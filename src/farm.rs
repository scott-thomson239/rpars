use std::{marker::PhantomData, sync::Arc};

use crate::{
    handler::Handler,
    rpar::{RPar, RParTemplate, Task},
};

pub struct Farm<TInput: Send> {
    cur_pos: usize,
    handlers: Vec<Handler<TInput>>,
}

impl<TInput: 'static + Send> Farm<TInput> {
    fn new<
        TOutput: 'static + Send,
        TRPar: 'static + Send + RPar<TInput>,
        FInner: FnOnce(Handler<TOutput>) -> TRPar,
        FOuter: FnOnce(Handler<TOutput>) -> Farm<TInput>,
    >(
        workers: i8,
        rpar_template: RParTemplate<TInput, TOutput, TRPar>,
    ) -> RParTemplate<TInput, TOutput, Farm<TInput>> {
        let worker_templates: Vec<RParTemplate<TInput, TOutput, TRPar>> = (0..workers).map(|_| rpar_template.clone()).collect();
        RParTemplate {
            f: Arc::new(Box::new(move |next_handler| Farm {
                cur_pos: 0,
                handlers: worker_templates.iter().map(|temp| Handler::new(temp.apply(next_handler.clone()))).collect()
            })),
            _marker: PhantomData
        }
    }
}

impl<TInput: 'static + Send> RPar<TInput> for Farm<TInput> {
    fn process(&mut self, task: Task<TInput>) -> Result<(), ()> {
        let len = self.handlers.len();
        let cur_handler = &mut self.handlers[self.cur_pos];
        self.cur_pos = (self.cur_pos + 1) % len;
        cur_handler.send(task)
    }
}
