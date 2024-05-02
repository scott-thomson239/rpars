pub trait Node {
    fn try_process_next(&mut self) -> Result<(), ()>;
}
