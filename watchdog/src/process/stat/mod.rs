#[cfg(target_os="macos")]
mod mac;
#[cfg(target_os="macos")]
use crate::process::stat::mac as sys;

#[cfg(target_os="windows")]
mod windows;
#[cfg(target_os="windows")]
use crate::process::stat::windows as sys;

#[cfg(target_os="linux")]
mod linux;
#[cfg(target_os="linux")]
use crate::process::stat::linux as sys;

pub fn processTimestamp(pid: i32) -> u64 {
    sys::processTimestamp(pid)
}
