extern crate watchdog;

use std::process::Command;
use sysinfo::{ProcessExt, SystemExt};

use rust_parse::cmd::CCmd;

use watchdog::config::CConfig;
use watchdog::process::check::CCheck;

const argConfigFile: &str = "-cfg";
const argCheckTime: &str = "-sleep";

fn main() {
    let mut cmdHandler = CCmd::new();
    let configFile = cmdHandler.register(argConfigFile, "watchdog.json");
    let checkTime = cmdHandler.register(argCheckTime, "3000");
    cmdHandler.parse();

    let configFile = configFile.borrow();
    let checkTime = checkTime.borrow();

    // read config file
    let config = CConfig::new();
    let info = config.read(&configFile);

    if let Ok(checkTime) = checkTime.parse::<u32>() {
        // start check
        let mut check = CCheck::new(info.process_list);
        check.start(checkTime);
    } else {
        println!("please input true sleep time");
    }
}
