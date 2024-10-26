pub enum TryProcessError {
    NodeCancelled,
    ProcessError,
}

pub trait Node: Send {
    fn try_process_next(&mut self) -> Result<(), TryProcessError>;
}
