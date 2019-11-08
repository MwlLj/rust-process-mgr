use crate::config::Process;
use super::kill;
use super::ProcessStatus;

#[cfg(all(not(target_os="windows"), not(target_arch="arm")))]
use sysinfo::{ProcessExt, SystemExt, System};
use chrono::prelude::*;

use std::thread;
use std::time;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::process::Command;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;

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
    pids: PidMapping,
    #[cfg(all(not(target_os="windows"), not(target_arch="arm")))]
    system: Arc<Mutex<System>>
}

impl CControl {
    pub fn startNewProcess(&self, name: &str) {
        let name = name.to_string();
        let mut processes = self.processes.clone();
        let mut pids = self.pids.clone();
        #[cfg(all(not(target_os="windows"), not(target_arch="arm")))]
        let mut system = self.system.clone();
        // get system PATH
        let systemPath = match env::var_os("PATH") {
            Some(p) => {
                match p.into_string() {
                    Ok(path) => path,
                    Err(err) => {
                        println!("into_string error, err: {:?}", err);
                        "".to_string()
                    }
                }
            },
            None => {
                println!("var_os error");
                "".to_string()
            }
        };
        std::thread::spawn(move || {
            let mut osPath = systemPath.clone();
            loop {
                let mut pids = pids.clone();
                match CControl::findProcess(&processes, &name, |process: &Process| -> RunResult {
                    println!("isAuto: {}", process.isAuto);
                    if process.isAuto == false {
                        return RunResult::IsnotAuto;
                    }
                    // CControl::writeLog(&(String::from("start process: ") + &name + ", time: " + &Local::now().timestamp().to_string() + "\n"));
                    // starting
                    CControl::replacePid(pids.clone(), &name, -1, ProcessStatus::Starting);
                    // calc start time
                    let startTime = Local::now().timestamp();
                    let mut execute = process.execute.clone();
                    // let mut argss = process.args.clone();
                    let mut args = Vec::new();
                    for arg in &process.args {
                        let ss = arg.split_whitespace();
                        for s in ss {
                            args.push(s.to_string());
                        }
                    }
                    if execute == "" {
                        if args.len() == 0 {
                            CControl::replacePid(pids.clone(), &name, -1, ProcessStatus::Failed("args error".to_string()));
                            return RunResult::Failed;
                        }
                        execute = args[0].to_string();
                        if args.len() > 1 {
                            args = args[1..].to_vec();
                        }
                    }
                    let mut commond = Command::new(execute.clone());
                    for arg in &args {
                        commond.arg(arg);
                    }
                    // join PATH
                    if cfg!(target_os="windows") {
                        osPath.push_str(";");
                    } else {
                        osPath.push_str(":");
                    }
                    osPath.push_str(&process.directory);
                    #[cfg(all(not(target_os="windows"), not(target_arch="arm")))]
                    CControl::killStartedProcess(system.clone(), &execute, &args, &process.directory);
                    // CControl::killStartedProcess(system.clone(), &name, &args, &process.directory);
                    let mut child = match commond
                    .env("PATH", &osPath)
                    .current_dir(&process.directory)
                    .spawn() {
                        Ok(c) => c,
                        Err(err) => {
                            CControl::replacePid(pids.clone(), &name, -1, ProcessStatus::Failed(err.to_string()));
                            println!("start process error, err: {}", err);
                            return RunResult::Failed;
                        }
                    };
                    let pid = child.id() as i32;
                    // running
                    CControl::replacePid(pids.clone(), &name, pid, ProcessStatus::Running);
                    match process.restartTimeS {
                        Some(timeS) => {
                            if timeS > 0 {
                            let s = Local::now().timestamp();
                                loop {
                                    match child.try_wait() {
                                        Ok(Some(status)) => {
                                            println!("process normal exit, name: {}", &process.name);
                                            break;
                                        },
                                        Ok(None) => {},
                                        Err(err) => {
                                            println!("process failed exit, name: {}", &process.name);
                                            break;
                                        }
                                    }
                                    if Local::now().timestamp() - s >= timeS {
                                        println!("process restartTime timeout, name: {}", &process.name);
                                        break;
                                    }
                                    std::thread::sleep(time::Duration::from_secs(1));
                                }
                            } else {
                                match child.wait() {
                                    Ok(_) => {
                                        println!("process success exit, name: {}", &process.name);
                                    },
                                    Err(err) => {
                                        println!("process failed exit, name: {}", &process.name);
                                    }
                                }
                            }
                        },
                        None => {
                            match child.wait() {
                                Ok(_) => {
                                    println!("process success exit, name: {}", &process.name);
                                },
                                Err(err) => {
                                    println!("process failed exit, name: {}", &process.name);
                                }
                            }
                        }
                    }
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

    pub fn stopProcess(&mut self, name: &str) -> Result<ProcessVec, &str> {
        let pid = match self.findPid(name) {
            Some(p) => p,
            None => {
                println!("findPid error");
                return Err("findPid error");
            }
        };
        match pid.status {
            ProcessStatus::Running => {
            },
            _ => {
                return Ok(self.processes.clone());
            }
        };
        self.cancelProcessAuto(name, false);
        println!("pid: {}", &pid.pid);
        if !self.kill(pid.pid) {
            // rollback
            self.cancelProcessAuto(name, true);
            return Err("kill error");
        }
        Ok(self.processes.clone())
    }

    pub fn stopProcessByAlias(&mut self, alias: &str) -> Result<(), &str> {
        let names = self.findProcessNamesByAlias(alias);
        let names = match &names {
            Some(ns) => ns,
            None => {
                return Err("findProcessNamesByAlias error");
            }
        };
        for name in names {
            self.stopProcess(name);
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

    pub fn restartProcess(&mut self, name: &str) -> Result<ProcessVec, &str> {
        let pid = match self.findPid(name) {
            Some(p) => {
            	match p.status {
            		ProcessStatus::Failed(_)
            		| ProcessStatus::QuickExit => {
		                println!("process Failed or QuickExit, restart, name: {}", name);
		                self.updateIsAuto(name, true);
		                self.startNewProcess(name);
		                return Ok(self.processes.clone());
            		},
            		_ => p
            	}
            },
            None => {
                println!("process is not exist, name: {}", name);
                self.updateIsAuto(name, true);
                self.startNewProcess(name);
                return Ok(self.processes.clone());
            }
        };
        match pid.status {
            ProcessStatus::Running => {
            },
            _ => {
                return Err("process is not running");
            }
        };
        if !self.kill(pid.pid) {
            return Err("kill error");
        }
        Ok(self.processes.clone())
    }

    pub fn restartProcessByAlias(&mut self, alias: &str) -> Result<(), &str> {
        let names = self.findProcessNamesByAlias(alias);
        let names = match &names {
            Some(ns) => ns,
            None => {
                return Err("findProcessNamesByAlias error");
            }
        };
        for name in names {
            self.restartProcess(name);
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
        // CControl::writeLog(&(String::from("watchdog start, time: ") + &Local::now().timestamp().to_string() + "\n"));
        let ctrl = CControl{
            processes: processes,
            pids: Arc::new(Mutex::new(HashMap::new())),
            #[cfg(all(not(target_os="windows"), not(target_arch="arm")))]
            system: Arc::new(Mutex::new(sysinfo::System::new()))
        };
        ctrl
    }
}

impl CControl {
    fn kill(&self, pid: i32) -> bool {
        kill::kill(pid, kill::Signal::Kill)
    }

    #[cfg(all(not(target_os="windows"), not(target_arch="arm")))]
    fn killStartedProcess(system: Arc<Mutex<System>>, name: &str, args: &Vec<String>, dir: &str) {
        let mut system = match system.lock() {
            Ok(s) => s,
            Err(err) => {
                println!("system lock error, err: {}", err);
                return;
            }
        };
        system.refresh_all();
        let processes = system.get_process_by_name(name);
        for process in processes {
            let processDir = match process.exe().parent() {
                Some(d) => d,
                None => {
                    continue;
                }
            };
            // println!("process cmd: {:?}\nargs: {:?}\nprocess exe: {:?}\ndir: {:?}", &process.cmd()[1..].to_vec(), &args, processDir, Path::new(dir));
            if &process.cmd()[1..].to_vec() == args
            && processDir == Path::new(dir) {
                println!("process starting ..., kill");
                process.kill(sysinfo::Signal::Kill);
            }
        }
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

    fn writeLog(content: &str) {
        let mut file = match OpenOptions::new().append(true).create(true).open("tmp.log") {
            Ok(f) => f,
            Err(err) => {
                println!("write log error, err: {}", err);
                return;
            }
        };
        file.write(content.as_bytes());
    }

    fn findProcessNamesByAlias(&self, alias: &str) -> Option<Vec<String>> {
        /*
        ** Find all process information by alias
        */
        let mut processNames = Vec::new();
        match self.processes.lock() {
            Ok(p) => {
                for process in p.iter() {
                    let ali = match &process.alias {
                        Some(a) => a,
                        None => {
                            continue;
                        }
                    };
                    if ali == alias {
                        processNames.push(process.name.clone());
                    }
                }
            },
            Err(err) => {
                return None;
            }
        }
        Some(processNames)
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
