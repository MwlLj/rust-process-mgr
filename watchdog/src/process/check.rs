extern crate sysinfo;

use std::thread;

use std::process::Command;
use std::sync::Arc;
use sysinfo::{ProcessExt, SystemExt, System, ProcessStatus, Signal};

use super::super::config::Process;

pub struct CCheck {
    // system: Arc<System>,
    system: System,
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
        // Arc::get_mut(&mut self.system).unwrap().refresh_all();
        self.system.refresh_all();
        for item in &(*self.processes) {
            // if self.system.get_process_by_name(&item.name).len() == 0 {
            //     if item.isAuto == true {
            //         if let Ok(_) = Command::new(&item.name)
            //         .args(&item.args)
            //         .env("PATH", &item.directory)
            //         .current_dir(&item.directory)
            //         .spawn() {
            //             println!("{} start success", &item.name);
            //         }
            //     }
            // }
            let ps = self.system.get_process_by_name(&item.name);
            for it in ps {
                let status = it.status();
                // if status == ProcessStatus::Zombie {
                    if it.kill(Signal::Child) {
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
                // }
            }
        }
    }

    pub fn new(processes: Arc<Vec<Process>>) -> CCheck {
        let system = System::new();
        let check = CCheck{
            system: system,
            processes: processes
        };
        check
    }
}
