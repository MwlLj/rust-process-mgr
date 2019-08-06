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

unsafe fn get_start_time(handle: HANDLE) -> u64 {
    let mut fstart: FILETIME = std::mem::zeroed();
    let mut x = std::mem::zeroed();

    GetProcessTimes(handle,
                    &mut fstart as *mut FILETIME,
                    &mut x as *mut FILETIME,
                    &mut x as *mut FILETIME,
                    &mut x as *mut FILETIME);
    let tmp = (fstart.dwHighDateTime as u64) << 32 | (fstart.dwLowDateTime as u64);
    tmp / 10_000_000 - 11_644_473_600
}

pub fn processTimestamp(pid: i32) -> u64 {
    let handle = match get_process_handler(pid) {
        Some(h) => h,
        None => {
            return 0;
        }
    };
    let handle = HandleWrapper(handle);
    if handle.is_null() {
        0
    } else {
        unsafe { get_start_time(*handle) }
    }
}
