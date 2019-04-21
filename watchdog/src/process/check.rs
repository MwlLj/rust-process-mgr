extern crate sysinfo;

use std::thread;

use std::process::Command;
use std::sync::Arc;
use sysinfo::{ProcessExt, SystemExt, System};

use super::super::config::Process;

pub struct CCheck {
    system: Arc<System>,
    processes: Arc<Vec<Process>>
}

impl CCheck {
    pub fn start(mut self, sleepTime: u32) {
        thread::spawn(move || {
            loop {
                self.findAndStartSubProcess();
                thread::sleep_ms(sleepTime);
            }
        });
    }

    fn findAndStartSubProcess(&mut self) {
        Arc::get_mut(&mut self.system).unwrap().refresh_all();
        for item in &(*self.processes) {
            if self.system.get_process_by_name(&item.name).len() == 0 {
                if item.isAuto == true {
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
    }

    pub fn new(system: Arc<System>, processes: Arc<Vec<Process>>) -> CCheck {
        let check = CCheck{
            system: system,
            processes: processes
        };
        check
    }
}
