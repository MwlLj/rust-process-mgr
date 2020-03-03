extern crate watchdog;

use std::thread;
use std::time;
use std::sync::Arc;
use std::sync::Mutex;
use std::default::Default;
use std::collections::VecDeque;
use std::io::prelude::*;
use std::fs::OpenOptions;

use rust_parse::cmd::CCmd;

use watchdog::config::file::CFile;
use watchdog::config::Process;
use watchdog::config::ConfigInfo;
use watchdog::process::control;
use watchdog::webserver::server::CServer;
use watchdog::core::dispatch::CDispatch;

const argConfigFile: &str = "-cfg";
const argCheckTime: &str = "-sleep";
const argHttpHost: &str = "-host";
const argHttpPort: &str = "-port";
const argUser: &str = "-user";
const argPwd: &str = "-pwd";
const argJsPath: &str = "-js-path";

struct CRun {
    config: ConfigInfo
}

impl CRun {
    fn run(mut self) {
        let mut cmdHandler = CCmd::new();
        let configFile = cmdHandler.register_with_desc(argConfigFile, "watchdog.json", "config file path, default watchdog.json, exp: watchdog.json");
        let checkTime = cmdHandler.register_with_desc(argCheckTime, "3000", "check sleep time, default 3000, exp: 3000");
        let httpHost = cmdHandler.register_with_desc(argHttpHost, "0.0.0.0", "http host, default 0.0.0.0, exp: 0.0.0.0");
        let httpPort = cmdHandler.register_with_desc(argHttpPort, "51000", "http port, default 51000, exp: 51000");
        let user = cmdHandler.register_with_desc(argUser, "admin", "user name, default admin, exp: admin");
        let pwd = cmdHandler.register_with_desc(argPwd, "123456", "http pwd, default 123456, exp: 123456");
        let jsPath = cmdHandler.register_with_desc(argJsPath, "js/jquery-3.3.1.min.js", "js path, default js/jquery-3.3.1.min.js, exp: js/jquery-3.3.1.min.js");
        cmdHandler.parse();

        let configFile = configFile.borrow();
        let checkTime = checkTime.borrow();
        let httpHost = httpHost.borrow();
        let httpPort = httpPort.borrow();
        let user = user.borrow();
        let pwd = pwd.borrow();
        let jsPath = jsPath.borrow();

        let httpPort = match httpPort.parse::<u32>() {
            Ok(p) => p,
            Err(err) => {
                println!("http port is not number, err: {}", err);
                return;
            }
        };

        let mut message = String::new();
        message.push_str("\tweb access way: http://ip:port/index\n");
        println!("{}", message);

        // writeLog(&(String::from("new dispatch start") + "\n"));
        let dispatch = CDispatch::new(&*configFile);
        // writeLog(&(String::from("new dispatch end") + "\n"));
        let mut server = CServer::new(dispatch);
        // writeLog(&(String::from("server start") + "\n"));
        server.start(&user, &pwd, &httpHost, httpPort, &jsPath);
        // writeLog(&(String::from("server end") + "\n"));
    }

    fn new() -> CRun {
        let run = CRun{
            config: ConfigInfo{
                processList: Default::default()
            }
        };
        run
    }
}

fn startNewProcessTest() {
    let mut processes = VecDeque::new();
    let name = "test1";
    let process = Process {
        name: name.to_string(),
        alias: Some("test1".to_string()),
        execute: "test".to_string(),
        args: Vec::new(),
        directory: ".".to_string(),
        isAuto: true,
        restartTimeS: Some(0)
    };
    processes.push_back(process);
    let mut ctrl = control::CControl::new(Arc::new(Mutex::new(processes)));
    ctrl.startNewProcess(&name);
    thread::sleep(time::Duration::from_secs(5));
    ctrl.stopProcess(&name);
    loop {
        thread::sleep(time::Duration::from_secs(60));
        ctrl.cancelProcessAuto(&name, false);
    }
}

fn dispatchTest() {
    let mut dispatch = CDispatch::new("test.json");
    dispatch.start();
    loop {
        thread::sleep(time::Duration::from_secs(1));
        let runTime = dispatch.getRunStatus("test", "test").unwrap();
        println!("runtime: {:?}", &runTime);
        /*
        let mut processes = VecDeque::new();
        let name = "test2";
        let process = Process {
            name: name.to_string(),
            serviceUuid: "".to_string(),
            args: Vec::new(),
            directory: ".".to_string(),
            isAuto: true
        };
        processes.push_back(process);
        dispatch.reload(&mut processes);
        */
    }
}

fn webServerTest() {
    let dispatch = CDispatch::new("test.json");
    let mut server = CServer::new(dispatch);
    server.start("admin", "123456", "0.0.0.0", 12345, "js/jquery-3.3.1.min.js");
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

fn main() {
    // writeLog(&(String::from("main start") + "\n"));
    let runner = CRun::new();
    runner.run();
    // startNewProcessTest();
    // dispatchTest();
    // webServerTest();
}
