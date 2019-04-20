extern crate sysinfo;

use std::thread;

use std::process::Command;
use sysinfo::{ProcessExt, SystemExt, System};

use super::super::config::Process;

pub struct CCheck {
    system: System,
    processes: Vec<Process>
}

impl CCheck {
    pub fn start(&mut self, sleepTime: u32) {
        loop {
            self.findAndStartSubProcess();
            thread::sleep_ms(sleepTime);
        }
    }

    fn findAndStartSubProcess(&mut self) {
        self.system.refresh_all();
        for item in &self.processes {
            if self.system.get_process_by_name(&item.name).len() == 0 {
                if let Ok(_) = Command::new(&item.name)
                .args(&item.args)
                .env("PATH", &item.directory)
                .current_dir(&item.directory)
                .spawn() {
                    println!("{} start success", &item.name);
                }
            }
        }
    }

    pub fn new(processes: Vec<Process>) -> CCheck {
        let system = sysinfo::System::new();
        let check = CCheck{
            system: system,
            processes: processes
        };
        check
    }
}
