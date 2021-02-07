#[derive(Debug)]
pub enum PartFrame<T: std::fmt::Debug> {
    End(T),
    Continue(T),
}

impl<T: std::fmt::Debug> PartFrame<T> {
    fn is_end(&self) -> bool {
        match self {
            PartFrame::End(_) => true,
            _ => false,
        }
    }
}
