use chrono::prelude::*;
use std::sync::{Arc, Mutex};
use sysinfo::{ProcessExt, SystemExt, System, Signal};

use std::thread;
use std::time;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::process::Command;
use std::collections::VecDeque;
use std::collections::HashMap;

use crate::config::Process;
use super::kill;
use super::ProcessStatus;

enum RunResult {
    True,
    IsnotAuto,
    Failed,
    Error,
    QuickExit,
    IsFind(bool)
}

#[derive(Clone, Debug)]
pub struct CPid {
    pub pid: i32,
    pub status: ProcessStatus
}

type ProcessVec = Arc<Mutex<VecDeque<Process>>>;
type PidMapping = Arc<Mutex<HashMap<String, CPid>>>;

pub struct CControl {
    processes: ProcessVec,
    pids: PidMapping
}

impl CControl {
    pub fn startNewProcess(&self, name: &str) {
        let name = name.to_string();
        let mut processes = self.processes.clone();
        let mut pids = self.pids.clone();
        std::thread::spawn(move || {
            loop {
                let mut pids = pids.clone();
                match CControl::findProcess(&processes, &name, |process: &Process| -> RunResult {
                    println!("isAuto: {}", process.isAuto);
                    if process.isAuto == false {
                        return RunResult::IsnotAuto;
                    }
                    // starting
                    CControl::replacePid(pids.clone(), &name, -1, ProcessStatus::Starting);
                    // calc start time
                    let startTime = Local::now().timestamp();
                    let mut execute = &process.execute;
                    let mut args = process.args.clone();
                    if execute == "" {
                        if process.args.len() == 0 {
                            CControl::replacePid(pids.clone(), &name, -1, ProcessStatus::Failed("args error".to_string()));
                            return RunResult::Failed;
                        }
                        execute = &process.args[0];
                        args = args[1..].to_vec();
                    }
                    let mut child = match Command::new(execute)
                    .args(args)
                    .env("PATH", &process.directory)
                    .current_dir(&process.directory)
                    .spawn() {
                        Ok(c) => c,
                        Err(err) => {
                            CControl::replacePid(pids.clone(), &name, -1, ProcessStatus::Failed(err.to_string()));
                            println!("start process error, err: {}", err);
                            return RunResult::Failed;
                        }
                    };
                    std::thread::sleep(time::Duration::from_millis(200));
                    let pid = child.id() as i32;
                    // running
                    CControl::replacePid(pids.clone(), &name, pid, ProcessStatus::Running);
                    match child.wait() {
                        Ok(_) => {
                            println!("process success exit, name: {}", &process.name);
                        },
                        Err(err) => {
                            println!("process failed exit, name: {}", &process.name);
                        }
                    }
                    CControl::killC(pid);
                    // calc stop time
                    let stopTime = Local::now().timestamp();
                    if stopTime - startTime < 3 {
                        CControl::replacePid(pids.clone(), &name, -1, ProcessStatus::QuickExit);
                        println!("process quick exit error");
                        return RunResult::Failed;
                    }
                    // stoped
                    CControl::replacePid(pids.clone(), &name, pid, ProcessStatus::Stoped);
                    return RunResult::True;
                }) {
                    RunResult::Error
                        | RunResult::IsnotAuto
                        | RunResult::IsFind(false) => {
                        // process exit -> delete from pids
                        println!("process thread exit, name: {}", &name);
                        CControl::deletePid(pids, &name);
                        break;
                    },
                    RunResult::Failed
                        | RunResult::QuickExit => {
                        // doesn't remove from pids
                        // => can query CPid, but doesn't run
                        println!("process start failed or quick exit");
                        break;
                    },
                    _ => {
                    }
                }
            }
        });
    }

    pub fn startAllProcess(&self) {
        let mut processNames = Vec::new();
        {
            let pros = match self.processes.lock() {
                Ok(p) => p,
                Err(err) => {
                    println!("processes lock error, err: {}", err);
                    return;
                }
            };
            for process in pros.iter() {
                processNames.push(process.name.to_string());
            }
        }
        for name in processNames {
            self.startNewProcess(&name);
        }
    }

    pub fn cancelProcessAuto(&mut self, name: &str, isAuto: bool) {
        let mut processes = match self.processes.lock() {
            Ok(p) => p,
            Err(err) => {
                println!("lock processess error, err: {}", err);
                return;
            }
        };
        for process in processes.iter_mut() {
            if process.name == name {
                process.isAuto = isAuto;
                break;
            }
        }
    }

    pub fn stopAllProcess(&mut self) {
        let mut processNames = Vec::new();
        {
            let pros = match self.processes.lock() {
                Ok(p) => p,
                Err(err) => {
                    println!("processes lock error, err: {}", err);
                    return;
                }
            };
            for process in pros.iter() {
                processNames.push(process.name.to_string());
            }
        }
        for name in processNames {
            self.stopProcess(&name);
        }
    }

    pub fn stopProcess(&mut self, name: &str) -> Result<(), &str> {
        let pid = match self.findPid(name) {
            Some(p) => p,
            None => {
                println!("findPid error");
                return Err("findPid error");
            }
        };
        self.cancelProcessAuto(name, false);
        if !self.kill(pid.pid) {
            // rollback
            self.cancelProcessAuto(name, true);
            return Err("kill error");
        }
        Ok(())
    }

    pub fn restartAllProcess(&mut self) {
        let mut processNames = Vec::new();
        {
            let pros = match self.processes.lock() {
                Ok(p) => p,
                Err(err) => {
                    println!("processes lock error, err: {}", err);
                    return;
                }
            };
            for process in pros.iter() {
                processNames.push(process.name.to_string());
            }
        }
        for name in processNames {
            self.restartProcess(&name);
        }
    }

    pub fn restartProcess(&mut self, name: &str) -> Result<(), &str> {
        let pid = match self.findPid(name) {
            Some(p) => {
            	match p.status {
            		ProcessStatus::Failed(_)
            		| ProcessStatus::QuickExit => {
		                println!("process Failed or QuickExit, restart, name: {}", name);
		                self.updateIsAuto(name, true);
		                self.startNewProcess(name);
		                return Ok(());
            		},
            		_ => p
            	}
            },
            None => {
                println!("process is not exist, name: {}", name);
                self.updateIsAuto(name, true);
                self.startNewProcess(name);
                return Ok(());
            }
        };
        if !self.kill(pid.pid) {
            return Err("kill error");
        }
        Ok(())
    }

    pub fn findPid(&self, name: &str) -> Option<CPid> {
        let pids = match self.pids.lock() {
            Ok(pids) => pids,
            Err(err) => {
                println!("pids lock error, err: {}", err);
                return None;
            }
        };
        if let Some(pid) = pids.get(name) {
            Some((*pid).clone())
        } else {
            None
        }
    }

    pub fn removeNameFromPidMapping(&mut self, name: &str) {
        let mut pids = match self.pids.lock() {
            Ok(pids) => pids,
            Err(err) => {
                println!("pids lock error, err: {}", err);
                return;
            }
        };
        pids.remove(name);
    }

    pub fn new(processes: ProcessVec) -> CControl {
        let ctrl = CControl{
            processes: processes,
            pids: Arc::new(Mutex::new(HashMap::new()))
        };
        ctrl
    }
}

impl CControl {
    fn kill(&self, pid: i32) -> bool {
        kill::kill(pid, kill::Signal::Kill)
    }

    fn killC(pid: i32) -> bool {
        kill::kill(pid, kill::Signal::Kill)
    }

    fn updateIsAuto(&self, name: &str, isAuto: bool) -> Result<(), &str> {
        let mut pros = match self.processes.lock() {
            Ok(p) => p,
            Err(err) => {
                println!("processes lock error, err: {}", err);
                return Err("processes lock error");
            }
        };
        for process in pros.iter_mut() {
            if process.name == name {
                (*process).isAuto = true;
                break;
            }
        }
        Ok(())
    }

    fn findProcess<F>(processes: &ProcessVec, name: &str, mut f: F) -> RunResult
        where F: FnOnce(&Process) -> RunResult {
        let mut pro = Process::default();
        let mut isFind = false;
        {
            let mut processes = match processes.lock() {
                Ok(p) => p,
                Err(err) => {
                    println!("lock processess error, err: {}", err);
                    return RunResult::Error;
                }
            };
            for process in processes.iter() {
                if process.name == name {
                    isFind = true;
                    pro = process.clone();
                    break;
                }
            }
        }
        if isFind {
            return f(&pro);
        }
        RunResult::IsFind(isFind)
    }

    fn replacePid(pids: PidMapping, name: &str, pid: i32, status: ProcessStatus) {
        let mut pids = match pids.lock() {
            Ok(pids) => pids,
            Err(err) => {
                println!("pids lock error, err: {}", err);
                return;
            }
        };
        if let Some(_) = pids.insert(name.to_string(), CPid{
            pid: pid,
            status: status.clone()
        }) {
            println!("update pid, name: {}, pid: {}, status: {:?}", name, pid, status);
        } else {
            println!("add pid, name: {}, pid: {}", name, pid);
        }
    }

    fn deletePid(pids: PidMapping, name: &str) {
        let mut pids = match pids.lock() {
            Ok(pids) => pids,
            Err(err) => {
                println!("pids lock error, err: {}", err);
                return;
            }
        };
        if let Some(_) = pids.remove(name) {
            println!("delete exist process, name: {}", name);
        } else {
            println!("delete failed, name: {} is not exist", name);
        }
    }
}

#[test]
fn startNewProcessTest() {
    let mut processes = VecDeque::new();
    let process = Process {
        name: "test".to_string(),
        serviceUuid: "".to_string(),
        args: Vec::new(),
        directory: ".".to_string(),
        isAuto: true
    };
    processes.push_back(process);
    let ctrl = CControl::new(Arc::new(Mutex::new(processes)));
    ctrl.startNewProcess("test");
}
