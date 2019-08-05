use crate::config::{Process};
use crate::tools::compare;
use crate::process::control;
use crate::process::status;
use crate::config::file;
use crate::config::ConfigInfo;

use sysinfo::{SystemExt, System};

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread;
use std::time;

type ProcessVec = Arc<Mutex<VecDeque<Process>>>;
type ProcessCtrl = Arc<Mutex<control::CControl>>;
type SystemArc = Arc<Mutex<System>>;

pub struct CDispatch {
    path: String,
    processes: ProcessVec,
    fileOps: file::CFile,
    processCtrl: ProcessCtrl,
    system: SystemArc,
    processStatus: status::CStatus
}

impl CDispatch {
    pub fn start(&mut self) {
        // load config file
        let mut processes = self.load();
        // update memory
        self.refreshProcesses(&mut processes);
        // start processes
        // thread::sleep(time::Duration::from_secs(3));
        self.processCtrl.lock().unwrap().startAllProcess();
    }

    pub fn reload(&mut self, news: &mut VecDeque<Process>) {
        let mut olds: VecDeque<Process> = Default::default();
        {
            let mut processes = match self.processes.lock() {
                Ok(p) => p,
                Err(err) => {
                    print!("processes lock error, err: {}", err);
                    return;
                }
            };
            olds = processes.clone();
        }
        self.refreshProcesses(news);
        let (adds, updates, deletes) = compare::processCompare(news, &olds, |o: &Process, n: &Process| -> bool {
            if o.isAuto != n.isAuto {
                return true;
            }
            return false;
        });
        println!("adds: {:?}", &adds);
        println!("updates: {:?}", &updates);
        println!("deletes: {:?}", &deletes);
        for process in adds.iter() {
            self.processCtrl.lock().unwrap().startNewProcess(&process.name);
        }
        for process in updates.iter() {
            self.processCtrl.lock().unwrap().restartProcess(&process.name);
        }
        for process in deletes.iter() {
            self.processCtrl.lock().unwrap().stopProcess(&process.name);
            // when process status is Failed or QuickExit -> pidMapping doesn't be delete
            self.processCtrl.lock().unwrap().removeNameFromPidMapping(&process.name);
        }
    }

    pub fn getAllRunStatus(&self) -> Result<Vec<status::CStatusInfo>, &str> {
        self.processStatus.getAllRunStatus(self.processes.clone())
    }

    pub fn getRunStatus(&self, name: &str) -> Result<status::CStatusInfo, &str> {
        self.processStatus.getRunStatus(name, true)
    }

    pub fn stopAllProcess(&mut self) {
        self.processCtrl.lock().unwrap().stopAllProcess();
    }

    pub fn stopProcess(&mut self, name: &str) -> Result<(), &str> {
        let mut ctrl = match self.processCtrl.lock() {
            Ok(c) => c,
            Err(err) => {
                println!("process ctrl lock error, err: {}", err);
                return Err("process ctrl lock error");
            }
        };
        if let Err(err) = ctrl.stopProcess(name) {
            return Err("stop process error");
        };
        Ok(())
    }

    pub fn restartAllProcess(&mut self) {
        self.processCtrl.lock().unwrap().restartAllProcess();
    }

    pub fn restartProcess(&mut self, name: &str) -> Result<(), &str> {
        let mut ctrl = match self.processCtrl.lock() {
            Ok(c) => c,
            Err(err) => {
                println!("process ctrl lock error, err: {}", err);
                return Err("process ctrl lock error");
            }
        };
        if let Err(err) = ctrl.restartProcess(name) {
            return Err("restart process error");
        };
        Ok(())
    }

    pub fn getConfigPath(&self) -> &str {
        return &self.path;
    }

    pub fn fileOps(&self) -> &file::CFile {
        return &self.fileOps;
    }
}

impl CDispatch {
    fn load(&self) -> VecDeque<Process> {
        let configInfo = self.fileOps.read().expect("load read error");
        configInfo.0.processList
    }

    fn refreshProcesses(&mut self, pros: &mut VecDeque<Process>) {
        let mut processes = match self.processes.lock() {
            Ok(p) => p,
            Err(err) => {
                println!("processes lock error, err: {}", err);
                return;
            }
        };
        // processes.clear();
        // processes.append(pros);
        *processes = pros.clone();
    }
}

impl CDispatch {
    pub fn new(path: &str) -> CDispatch {
        // System::new();
        let fileOps = file::CFile::new(path);
        let processes = Arc::new(Mutex::new(VecDeque::new()));
        let system = Arc::new(Mutex::new(System::new()));
        let processCtrl = control::CControl::new(processes.clone());
        let processCtrl = Arc::new(Mutex::new(processCtrl));
        let processStatus = status::CStatus::new(processCtrl.clone()
            , system.clone());
        CDispatch{
            path: path.to_string(),
            processes: processes.clone(),
            fileOps: fileOps,
            processCtrl: processCtrl.clone(),
            system: system.clone(),
            processStatus: processStatus
        }
    }
}
