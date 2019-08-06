use libc;
use std::ffi::CStr;

pub fn processTime(pid: i32) -> u64 {
    let mut path = String::new();
    path.push_str("/proc/");
    path.push_str(&pid.to_string());
    path.push('\0');
    let path = match CStr::from_bytes_with_nul(path.as_bytes()) {
        Ok(p) => p,
        Err(err) => {
            println!("from bytes with nul error, err: {}", err);
            return 0;
        }
    };
    let path = path.as_ptr();
    let stat = unsafe {
        let mut stat: libc::stat = unsafe { std::mem::zeroed() };
        libc::stat(path, &mut stat as *mut libc::stat);
        stat
    };
    stat.st_atime
}
