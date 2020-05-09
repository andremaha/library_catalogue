#[derive(Debug, Copy, Clone)]
pub enum Read {
    Finished,
    Reading(u32),
    Started,
    NotStarted
}