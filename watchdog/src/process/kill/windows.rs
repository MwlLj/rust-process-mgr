extern crate winapi;

use libc::{c_uint, c_void, memcpy};

use winapi::shared::minwindef::{DWORD, FALSE, FILETIME, MAX_PATH/*, TRUE, USHORT*/};
use winapi::um::winnt::{
    HANDLE, ULARGE_INTEGER, /*THREAD_GET_CONTEXT, THREAD_QUERY_INFORMATION, THREAD_SUSPEND_RESUME,*/
    /*, PWSTR*/ PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE, PROCESS_VM_READ,
};
use winapi::um::processthreadsapi::{GetProcessTimes, OpenProcess, TerminateProcess};

#[derive(Clone)]
struct HandleWrapper(HANDLE);

impl std::ops::Deref for HandleWrapper {
    type Target = HANDLE;

    fn deref(&self) -> &HANDLE {
        &self.0
    }
}

unsafe impl Send for HandleWrapper {}
unsafe impl Sync for HandleWrapper {}

fn get_process_handler(pid: i32) -> Option<HANDLE> {
    if pid == 0 {
        return None;
    }
    let options = PROCESS_QUERY_INFORMATION | PROCESS_VM_READ | PROCESS_TERMINATE;
    let process_handler = unsafe { OpenProcess(options, FALSE, pid as DWORD) };
    if process_handler.is_null() {
        let options = PROCESS_QUERY_INFORMATION | PROCESS_VM_READ;
        let process_handler = unsafe { OpenProcess(options, FALSE, pid as DWORD) };
        if process_handler.is_null() {
            None
        } else {
            Some(process_handler)
        }
    } else {
        Some(process_handler)
    }
}

pub fn kill(pid: i32, signal: super::Signal) -> bool {
    let handle = match get_process_handler(pid) {
        Some(h) => h,
        None => {
            return false
        }
    };
    let handle = HandleWrapper(handle);
    if handle.is_null() {
        false
    } else {
        unsafe { TerminateProcess(*handle, signal as c_uint) != 0 }
    }
}
