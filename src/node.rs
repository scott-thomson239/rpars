#[derive(Debug, PartialEq)]
pub enum TryProcessError {
    NodeCancelled,
    ProcessError,
    Empty,
}

pub trait Node: Send {
    fn try_process_next(&mut self) -> Result<(), TryProcessError>;
}
