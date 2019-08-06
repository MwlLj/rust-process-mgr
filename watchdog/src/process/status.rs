use crate::config::Process;
use crate::process::control;
use super::ProcessStatus;
use super::stat;

use chrono::prelude::*;
use chrono::{Duration, DateTime, NaiveDateTime};

use libc;

use std::sync::{Arc, Mutex};
use std::process::{Command, Stdio};
use std::collections::VecDeque;
use std::ffi::CStr;

type ProcessControl = Arc<Mutex<control::CControl>>;
// type SystemArc = Arc<Mutex<System>>;
type ProcessVec = Arc<Mutex<VecDeque<Process>>>;

const process_runtime_default: &str = "unknow";

#[derive(Debug)]
pub struct CStatusInfo {
    pub pid: i32,
    pub runTime: String,
    pub status: ProcessStatus,
    pub name: String
}

pub struct CStatus {
    processCtrl: ProcessControl
}

impl CStatus {
    pub fn getAllRunStatus(&self, processes: ProcessVec) -> Result<Vec<CStatusInfo>, &str> {
        /*
        {
            let mut system = match self.system.lock() {
                Ok(s) => s,
                Err(err) => {
                    println!("system lock error, err: {}", err);
                    return Err("system lock error");
                }
            };
            system.refresh_all();
        }
        */
        let mut processNames = Vec::new();
        {
            let pros = match processes.lock() {
                Ok(p) => p,
                Err(err) => {
                    println!("processes lock error, err: {}", err);
                    return Err("processes lock error");
                }
            };
            for process in pros.iter() {
                processNames.push(process.name.to_string());
            }
        }
        let mut statuses = Vec::new();
        for name in processNames {
            match self.getRunStatus(&name) {
                Ok(s) => {
                    statuses.push(s);
                },
                Err(err) => {
                    println!("getRunStatus error, err: {}", err);
                    continue;
                }
            }
        }
        Ok(statuses)
    }

    pub fn getRunStatus(&self, name: &str) -> Result<CStatusInfo, &str> {
        let pid = match self.findPidByName(name) {
            Ok(id) => id,
            Err(err) => {
                return Err(err);
            }
        };
        /*
        let mut system = match self.system.lock() {
            Ok(s) => s,
            Err(err) => {
                println!("system lock error, err: {}", err);
                return Err("system lock error");
            }
        };
        if !isRefresh {
            system.refresh_all();
        }
        let pro = match system.get_process(pid.pid as Pid) {
            Some(p) => p,
            None => {
                println!("process object not found");
                return Ok(CStatusInfo{
                    pid: pid.pid,
                    runTime: process_runtime_default.to_string(),
                    status: pid.status,
                    name: name.to_string()
                });
            }
        };
        */
        let mut procStatrTime = stat::processTimestamp(pid.pid) as i64;
        /*
        if (cfg!(all(target_os="linux", target_arch="arm"))) {
            if let Ok(output) = Command::new("stat")
                .arg(path)
                .stdout(Stdio::piped())
                .output() {
                let result = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = result.split("\n").collect();
                if lines.len() >= 5 {
                    let access = lines[4].trim();
                    let (key, value) = access.split_at("Access:".to_string().len());
                    let v = value.trim();
                    let timePA: Vec<&str> = v.split(".").collect();
                    if timePA.len() >= 1 {
                        let t = timePA[0];
                        let parse_from_str = NaiveDateTime::parse_from_str;
                        if let Ok(d) = parse_from_str(t, "%Y-%m-%d %H:%M:%S") {
                            procStatrTime = d.timestamp();
                        }
                    }
                }
            };
        } else {
            procStatrTime = pro.start_time() as i64;
        }
        */
        let dt = Local::now();
        let now = dt.timestamp();
        let sub = now - procStatrTime;
        let runTime = self.calcSec2DHMS(sub);
        Ok(CStatusInfo{
            pid: pid.pid,
            runTime: runTime,
            status: pid.status,
            name: name.to_string()
        })
    }
}

impl CStatus {
    // fn findProcessByPid<'a>(&'a self, pid: i32) -> Result<&'a sysinfo::Process, &str> {
    //     let system = match self.system.lock() {
    //         Ok(s) => s,
    //         Err(err) => {
    //             println!("system lock error, err: {}", err);
    //             return Err("system lock error");
    //         }
    //     };
    //     let pro = match system.get_process(pid as usize) {
    //         Some(p) => p,
    //         None => {
    //             println!("process object not found");
    //             return Err("process object not found");
    //         }
    //     };
    //     Ok(&*pro)
    // }

    fn findPidByName(&self, name: &str) -> Result<control::CPid, &str> {
        let ctrl = match self.processCtrl.lock() {
            Ok(c) => c,
            Err(err) => {
                println!("process ctrl lock error, err: {}", err);
                return Err("process ctrl lock error");
            }
        };
        let pid = match ctrl.findPid(name) {
            Some(pid) => pid,
            None => {
                println!("pid is not found, name: {}", name);
                return Ok(control::CPid{
                    pid: -1,
                    // status: ProcessStatus::Unknow
                    status: ProcessStatus::Stoped
                });
            }
        };
        Ok(pid)
    }

    fn calcSec2DHMS(&self, sec: i64) -> String {
        let mut result = String::new();
        let dur = Duration::seconds(sec);
        result.push_str(&dur.num_days().to_string());
        result.push_str("day, ");
        result.push_str(&(dur.num_hours()%24).to_string());
        result.push_str(":");
        result.push_str(&(dur.num_minutes()%60).to_string());
        result.push_str(":");
        result.push_str(&(dur.num_seconds()%60).to_string());
        result
    }
}

impl CStatus {
    pub fn new(ctrl: ProcessControl) -> CStatus {
        CStatus{
            processCtrl: ctrl
        }
    }
}
