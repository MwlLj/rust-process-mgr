extern crate watchdog;

use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::default::Default;

use rust_parse::cmd::CCmd;

use watchdog::config::CConfig;
use watchdog::config::Process;
use watchdog::config::ConfigInfo;
use watchdog::process::check::CCheck;
use watchdog::webserver::server::CServer;

const argConfigFile: &str = "-cfg";
const argCheckTime: &str = "-sleep";
const argHttpHost: &str = "-host";
const argHttpPort: &str = "-port";

struct CRun {
    config: ConfigInfo
}

impl CRun {
    fn run(mut self) {
        let mut message = String::new();
        message.push_str("options:\n");
        message.push_str("\t-cfg: config file path, default watchdog.json, exp: watchdog.json\n");
        message.push_str("\t-sleep: check sleep time, default 3000, exp: 3000\n");
        message.push_str("\t-host: http host, default 0.0.0.0, exp: 0.0.0.0\n");
        message.push_str("\t-port: http port, default 51000, exp: 51000\n");
        println!("{}", message);

        let mut cmdHandler = CCmd::new();
        let configFile = cmdHandler.register(argConfigFile, "watchdog.json");
        let checkTime = cmdHandler.register(argCheckTime, "3000");
        let httpHost = cmdHandler.register(argHttpHost, "0.0.0.0");
        let httpPort = cmdHandler.register(argHttpPort, "51000");
        cmdHandler.parse();

        let configFile = configFile.borrow();
        let checkTime = checkTime.borrow();
        let httpHost = httpHost.borrow();
        let httpPort = httpPort.borrow();

        // read config file
        let config = CConfig::new();
        self.config = config.read(&configFile);

        if let Ok(checkTime) = checkTime.parse::<u32>() {
            // start check
            let mut check = CCheck::new(self.config.process_list.clone());
            check.start(checkTime);
            // start http server
            if let Ok(httpPort) = httpPort.parse::<u32>() {
                println!("http server start success");
                let server = CServer::new(self.config.process_list.clone());
                server.start(&httpHost, httpPort);
            }
        } else {
            println!("please input true sleep time");
        }
    }

    fn new() -> CRun {
        let run = CRun{
            config: ConfigInfo{
                process_list: Default::default()
            }
        };
        run
    }
}

fn main() {
    let runner = CRun::new();
    runner.run();
}
