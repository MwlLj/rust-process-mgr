extern crate watchdog;

use std::thread;
use std::time;
use std::sync::Arc;
use std::sync::Mutex;
use std::default::Default;
use std::collections::VecDeque;

use sysinfo::{ProcessExt, SystemExt, System};
use rust_parse::cmd::CCmd;

use watchdog::config::file::CFile;
use watchdog::config::Process;
use watchdog::config::ConfigInfo;
use watchdog::process::check::CCheck;
use watchdog::process::control;
use watchdog::webserver::server::CServer;
use watchdog::core::dispatch::CDispatch;

const argConfigFile: &str = "-cfg";
const argCheckTime: &str = "-sleep";
const argHttpHost: &str = "-host";
const argHttpPort: &str = "-port";
const argPwd: &str = "-pwd";

struct CRun {
    config: ConfigInfo
}

impl CRun {
    fn run(mut self) {
        /*
        let mut message = String::new();
        message.push_str("options:\n");
        message.push_str("\t-cfg: config file path, default watchdog.json, exp: watchdog.json\n");
        message.push_str("\t-sleep: check sleep time, default 3000, exp: 3000\n");
        message.push_str("\t-host: http host, default 0.0.0.0, exp: 0.0.0.0\n");
        message.push_str("\t-port: http port, default 51000, exp: 51000\n");
        message.push_str("\t-pwd: http pwd, default 123456, exp: 123456\n");
        message.push_str("\tweb access way: http:/ip:port/index?pwd=123456\n");
        println!("{}", message);

        let mut cmdHandler = CCmd::new();
        let configFile = cmdHandler.register(argConfigFile, "watchdog.json");
        let checkTime = cmdHandler.register(argCheckTime, "3000");
        let httpHost = cmdHandler.register(argHttpHost, "0.0.0.0");
        let httpPort = cmdHandler.register(argHttpPort, "51000");
        let pwd = cmdHandler.register(argPwd, "123456");
        cmdHandler.parse();

        let configFile = configFile.borrow();
        let checkTime = checkTime.borrow();
        let httpHost = httpHost.borrow();
        let httpPort = httpPort.borrow();
        let pwd = pwd.borrow();

        // read config file
        let config = CFile::new(&*configFile);
        self.config = config.read();

        // init system
        // let system = sysinfo::System::new();

        if let Ok(checkTime) = checkTime.parse::<u32>() {
            let mut processList = Arc::new(Mutex::new(self.config.processList));
            // let mut system = Arc::new(system);
            // start check
            let mut check = CCheck::new(processList.clone());
            // let mut check = CCheck::new(Arc::clone(&mut system), Arc::clone(&mut processList));
            check.start(checkTime);
            // start http server
            if let Ok(httpPort) = httpPort.parse::<u32>() {
                println!("http server start success");
                let mut server = CServer::new(processList.clone());
                // let mut server = CServer::new(Arc::clone(&mut system), Arc::clone(&mut processList));
                server.start(&pwd, &httpHost, httpPort);
            }
        } else {
            println!("please input true sleep time");
        }
        */
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
        execute: "test".to_string(),
        args: Vec::new(),
        directory: ".".to_string(),
        isAuto: true
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
        let runTime = dispatch.getRunStatus("test").unwrap();
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
    server.start("123456", "0.0.0.0", 12345, "js/jquery-3.3.1.min.js");
}

fn main() {
    // let runner = CRun::new();
    // runner.run();
    // startNewProcessTest();
    // dispatchTest();
    webServerTest();
}
