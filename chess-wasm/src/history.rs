#[derive(Debug)]
pub struct History<E> {
    entries: Vec<E>,
}

pub trait HistoryEntry {}
