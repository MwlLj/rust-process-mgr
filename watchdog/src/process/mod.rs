#[derive(Clone, Debug)]
pub enum ProcessStatus {
    Starting,
    Running,
    Stoped,
    Failed(String),
    QuickExit,
    Unknow
}

const process_status_starting: &str = "starting";
const process_status_running: &str = "running";
const process_status_stoped: &str = "stoped";
const process_status_failed: &str = "failed";
const process_status_quickexit: &str = "quick_exit";
const process_status_unknow: &str = "unknow";

pub fn to_status_desc(status: &ProcessStatus) -> String {
	match status {
		ProcessStatus::Starting => {
			return String::from(process_status_starting);
		},
		ProcessStatus::Running => {
			return String::from(process_status_running);
		},
		ProcessStatus::Stoped => {
			return String::from(process_status_stoped);
		},
		ProcessStatus::Failed(_) => {
			return String::from(process_status_failed);
		},
		ProcessStatus::QuickExit => {
			return String::from(process_status_quickexit);
		},
		ProcessStatus::Unknow => {
			return String::from(process_status_unknow);
		}
	}
}

pub mod check;
pub mod control;
pub mod kill;
pub mod status;
