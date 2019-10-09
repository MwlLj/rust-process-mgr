use rust_parse::cmd::CCmd;
use watchdog_client::client;

const stopall_key: &str = "stopall";
const restartall_key: &str = "restartall";
const stop_key: &str = "stop";
const stop_by_alias_key: &str = "stopbyalias";
const restart_key: &str = "restart";
const restart_by_alias_key: &str = "restartbyalias";
const get_all_process_status_key: &str = "states";
const name_key: &str = "name";

fn main() {
    let mut cmdHandler = CCmd::new();
    let addr = cmdHandler.register_with_desc("-server", "127.0.0.1:51891", "");
    let stopall = cmdHandler.register_with_desc(stopall_key, "", "stop all");
    let restartall = cmdHandler.register_with_desc(restartall_key, "", "restart all");
    let stop = cmdHandler.register_with_desc(stop_key, "", "stop process by name");
    let stopbyalias = cmdHandler.register_with_desc(stop_by_alias_key, "", "stop process by alias");
    let restart = cmdHandler.register_with_desc(restart_key, "", "restart process by name");
    let restartbyalias = cmdHandler.register_with_desc(restart_by_alias_key, "", "restart process by alias");
    let states = cmdHandler.register_with_desc(get_all_process_status_key, "", "get all process status");
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
    } else if cmdHandler.has(stop_key) {
        if let Err(err) = client::stop::stop(&*addr, &*stop.borrow()) {
            println!("stop error, name: {}, err:{}", &*stop.borrow(), err);
            return;
        };
    } else if cmdHandler.has(stop_by_alias_key) {
        if let Err(err) = client::stop::stop_by_alias(&*addr, &*stopbyalias.borrow()) {
            println!("stop by alias error, alias: {}, err: {}", &*stopbyalias.borrow(), err);
            return;
        };
    } else if cmdHandler.has(restart_key) {
        if let Err(err) = client::restart::restart(&*addr, &*restart.borrow()) {
            println!("restart error, name: {}, err: {}", &*restart.borrow(), err);
            return;
        }
    } else if cmdHandler.has(restart_by_alias_key) {
        if let Err(err) = client::restart::restart_by_alias(&*addr, &*restartbyalias.borrow()) {
            println!("restart by aliaas error, alias: {}, err: {}", &*restartbyalias.borrow(), err);
            return;
        };
    } else if cmdHandler.has(get_all_process_status_key) {
        if let Err(err) = client::status::all_process_status(&*addr) {
            println!("get all process status error, err: {}", err);
            return;
        };
    }
}
