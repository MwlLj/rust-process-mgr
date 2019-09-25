use rust_parse::cmd::CCmd;
use watchdog_client::client;

const stopall_key: &str = "stopall";
const restartall_key: &str = "restartall";

fn main() {
    let mut cmdHandler = CCmd::new();
    let addr = cmdHandler.register_with_desc("-server", "127.0.0.1:51891", "");
    let stopall = cmdHandler.register_with_desc(stopall_key, "", "stop all");
    let restartall = cmdHandler.register_with_desc(restartall_key, "", "restart all");
    cmdHandler.parse();

    let addr = addr.borrow();

    if cmdHandler.has(stopall_key) && cmdHandler.has(restartall_key) {
        println!("stop and restart conflict");
        return;
    }

    if cmdHandler.has(stopall_key) {
        if let Err(err) = client::stop::stop_all(&*addr) {
            println!("stop_all error, err: {}", err);
            return;
        };
    } else if cmdHandler.has(restartall_key) {
        if let Err(err) = client::restart::restart_all(&*addr) {
            println!("restart_all error, err: {}", err);
            return;
        };
    }
}
