use libc::{c_int, gid_t, uid_t};

use std::cmp::PartialEq;

pub fn kill(pid: i32, signal: super::Signal) -> bool {
    unsafe { libc::kill(pid, signal as c_int) == 0 }
}
