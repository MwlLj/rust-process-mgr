#[derive(Clone, Debug)]
pub enum ProcessStatus {
    Starting,
    Running,
    Stoped,
    Failed,
    QuickExit,
    Unknow
}

pub mod check;
pub mod control;
pub mod kill;
pub mod status;
