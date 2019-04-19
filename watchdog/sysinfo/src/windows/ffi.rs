// 
// Sysinfo
// 
// Copyright (c) 2015 Guillaume Gomez
//

use libc::{c_int, c_char, c_void, c_uchar, c_uint, size_t, uint32_t, uint64_t, c_ushort};

extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;

    pub fn proc_pidinfo(pid: c_int, flavor: c_int, arg: u64, buffer: *mut c_void,
                        buffersize: c_int) -> c_int;
    pub fn proc_listallpids(buffer: *mut c_void, buffersize: c_int) -> c_int;
    //pub fn proc_name(pid: c_int, buffer: *mut c_void, buffersize: u32) -> c_int;
    //pub fn proc_regionfilename(pid: c_int, address: u64, buffer: *mut c_void,
    //                           buffersize: u32) -> c_int;
    //pub fn proc_pidpath(pid: c_int, buffer: *mut c_void, buffersize: u32) -> c_int;

    pub fn IOMasterPort(a: i32, b: *mut mach_port_t) -> i32;
    pub fn IOServiceMatching(a: *const c_char) -> *mut c_void;
    pub fn IOServiceGetMatchingServices(a: mach_port_t, b: *mut c_void, c: *mut io_iterator_t) -> i32;
    pub fn IOIteratorNext(iterator: io_iterator_t) -> io_object_t;
    pub fn IOObjectRelease(obj: io_object_t) -> i32;
    pub fn IOServiceOpen(device: io_object_t, a: u32, t: u32, x: *mut io_connect_t) -> i32;
    pub fn IOServiceClose(a: io_connect_t) -> i32;
    pub fn IOConnectCallStructMethod(connection: mach_port_t, 
                                     selector: u32, 
                                     inputStruct: *mut KeyData_t, 
                                     inputStructCnt: size_t, 
                                     outputStruct: *mut KeyData_t, 
                                     outputStructCnt: *mut size_t) -> i32;
    pub fn IORegistryEntryCreateCFProperties(entry: io_registry_entry_t,
                                             properties: *mut CFMutableDictionaryRef,
                                             allocator: CFAllocatorRef,
                                             options: IOOptionBits)
                                             -> kern_return_t;
    pub fn CFDictionaryContainsKey(d: CFDictionaryRef, key: *const c_void) -> Boolean;
    pub fn CFDictionaryGetValue(d: CFDictionaryRef, key: *const c_void) -> *const c_void;
    pub fn IORegistryEntryGetName(entry: io_registry_entry_t, name: *mut c_char) -> kern_return_t;
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFStringCreateWithCStringNoCopy(alloc: *mut c_void, cStr: *const c_char,
                                           encoding: CFStringEncoding,
                                           contentsDeallocator: *mut c_void) -> CFStringRef;

    pub static kCFAllocatorNull: CFAllocatorRef;

    pub fn mach_absolute_time() -> u64;
    //pub fn task_for_pid(host: u32, pid: pid_t, task: *mut task_t) -> u32;
    pub fn mach_task_self() -> u32;
    pub fn mach_host_self() -> u32;
    //pub fn task_info(host_info: u32, t: u32, c: *mut c_void, x: *mut u32) -> u32;
    pub fn host_statistics64(host_info: u32, x: u32, y: *mut c_void, z: *const u32) -> u32;
    pub fn host_processor_info(host_info: u32, t: u32, num_cpu_u: *mut u32,
                               cpu_info: *mut *mut i32, num_cpu_info: *mut u32) -> u32;
    //pub fn host_statistics(host_priv: u32, flavor: u32, host_info: *mut c_void,
    //                       host_count: *const u32) -> u32;
    pub fn vm_deallocate(target_task: u32, address: *mut i32, size: u32) -> u32;
}

// TODO: waiting for https://github.com/rust-lang/libc/pull/678
macro_rules! cfg_if {
    ($(
        if #[cfg($($meta:meta),*)] { $($it:item)* }
    ) else * else {
        $($it2:item)*
    }) => {
        __cfg_if_items! {
            () ;
            $( ( ($($meta),*) ($($it)*) ), )*
            ( () ($($it2)*) ),
        }
    }
}

// TODO: waiting for https://github.com/rust-lang/libc/pull/678
macro_rules! __cfg_if_items {
    (($($not:meta,)*) ; ) => {};
    (($($not:meta,)*) ; ( ($($m:meta),*) ($($it:item)*) ), $($rest:tt)*) => {
        __cfg_if_apply! { cfg(all(not(any($($not),*)), $($m,)*)), $($it)* }
        __cfg_if_items! { ($($not,)* $($m,)*) ; $($rest)* }
    }
}

// TODO: waiting for https://github.com/rust-lang/libc/pull/678
macro_rules! __cfg_if_apply {
    ($m:meta, $($it:item)*) => {
        $(#[$m] $it)*
    }
}

// TODO: waiting for https://github.com/rust-lang/libc/pull/678
cfg_if! {
    if #[cfg(any(target_arch = "arm", target_arch = "x86"))] {
        pub type timeval32 = ::libc::timeval;
    } else {
        use libc::timeval32;
    }
}

// TODO: waiting for https://github.com/rust-lang/libc/pull/678
#[repr(C)]
pub struct if_data64 {
    pub ifi_type: c_uchar,
    pub ifi_typelen: c_uchar,
    pub ifi_physical: c_uchar,
    pub ifi_addrlen: c_uchar,
    pub ifi_hdrlen: c_uchar,
    pub ifi_recvquota: c_uchar,
    pub ifi_xmitquota: c_uchar,
    pub ifi_unused1: c_uchar,
    pub ifi_mtu: uint32_t,
    pub ifi_metric: uint32_t,
    pub ifi_baudrate: uint64_t,
    pub ifi_ipackets: uint64_t,
    pub ifi_ierrors: uint64_t,
    pub ifi_opackets: uint64_t,
    pub ifi_oerrors: uint64_t,
    pub ifi_collisions: uint64_t,
    pub ifi_ibytes: uint64_t,
    pub ifi_obytes: uint64_t,
    pub ifi_imcasts: uint64_t,
    pub ifi_omcasts: uint64_t,
    pub ifi_iqdrops: uint64_t,
    pub ifi_noproto: uint64_t,
    pub ifi_recvtiming: uint32_t,
    pub ifi_xmittiming: uint32_t,
    pub ifi_lastchange: timeval32,
}

// TODO: waiting for https://github.com/rust-lang/libc/pull/678
#[repr(C)]
pub struct if_msghdr2 {
    pub ifm_msglen: c_ushort,
    pub ifm_version: c_uchar,
    pub ifm_type: c_uchar,
    pub ifm_addrs: c_int,
    pub ifm_flags: c_int,
    pub ifm_index: c_ushort,
    pub ifm_snd_len: c_int,
    pub ifm_snd_maxlen: c_int,
    pub ifm_snd_drops: c_int,
    pub ifm_timer: c_int,
    pub ifm_data: if_data64,
}

#[repr(C)]
pub struct __CFAllocator {
    __private: c_void,
}

#[repr(C)]
pub struct __CFDictionary {
    __private: c_void,
}

#[repr(C)]
pub struct __CFString {
    __private: c_void,
}

pub type CFAllocatorRef = *const __CFAllocator;
pub type CFMutableDictionaryRef = *mut __CFDictionary;
pub type CFDictionaryRef = *const __CFDictionary;
#[allow(non_camel_case_types)]
pub type io_name_t = [u8; 128];
#[allow(non_camel_case_types)]
pub type io_registry_entry_t = io_object_t;
pub type CFTypeRef = *const c_void;
pub type CFStringRef = *const __CFString;

//#[allow(non_camel_case_types)]
//pub type policy_t = i32;
#[allow(non_camel_case_types)]
//pub type integer_t = i32;
//#[allow(non_camel_case_types)]
//pub type time_t = i64;
//#[allow(non_camel_case_types)]
//pub type suseconds_t = i32;
//#[allow(non_camel_case_types)]
//pub type mach_vm_size_t = u64;
//#[allow(non_camel_case_types)]
//pub type task_t = u32;
//#[allow(non_camel_case_types)]
//pub type pid_t = i32;
#[allow(non_camel_case_types)]
pub type natural_t = u32;
#[allow(non_camel_case_types)]
pub type mach_port_t = u32;
#[allow(non_camel_case_types)]
pub type io_object_t = mach_port_t;
#[allow(non_camel_case_types)]
pub type io_iterator_t = io_object_t;
#[allow(non_camel_case_types)]
pub type io_connect_t = io_object_t;
#[allow(non_camel_case_types)]
pub type boolean_t = c_uint;
#[allow(non_camel_case_types)]
pub type kern_return_t = c_int;
pub type Boolean = c_uchar;
pub type IOOptionBits = u32;
pub type CFStringEncoding = u32;

/*#[repr(C)]
pub struct task_thread_times_info {
    pub user_time: time_value,
    pub system_time: time_value,
}*/

/*#[repr(C)]
pub struct task_basic_info_64 {
    pub suspend_count: integer_t,
    pub virtual_size: mach_vm_size_t,
    pub resident_size: mach_vm_size_t,
    pub user_time: time_value_t,
    pub system_time: time_value_t,
    pub policy: policy_t,
}*/

#[repr(C)]
pub struct vm_statistics64 {
    pub free_count: natural_t,
    pub active_count: natural_t,
    pub inactive_count: natural_t,
    pub wire_count: natural_t,
    pub zero_fill_count: u64,
    pub reactivations: u64,
    pub pageins: u64,
    pub pageouts: u64,
    pub faults: u64,
    pub cow_faults: u64,
    pub lookups: u64,
    pub hits: u64,
    pub purges: u64,
    pub purgeable_count: natural_t,
    pub speculative_count: natural_t,
    pub decompressions: u64,
    pub compressions: u64,
    pub swapins: u64,
    pub swapouts: u64,
    pub compressor_page_count: natural_t,
    pub throttled_count: natural_t,
    pub external_page_count: natural_t,
    pub internal_page_count: natural_t,
    pub total_uncompressed_pages_in_compressor: u64,
}

#[repr(C)]
pub struct Val_t {
    pub key: [i8; 5],
    pub data_size: u32,
    pub data_type: [i8; 5], // UInt32Char_t
    pub bytes: [i8; 32], // SMCBytes_t
}

#[repr(C)]
pub struct KeyData_vers_t {
    pub major: u8,
    pub minor: u8,
    pub build: u8,
    pub reserved: [u8; 1],
    pub release: u16,
}

#[repr(C)]
pub struct KeyData_pLimitData_t {
    pub version: u16,
    pub length: u16,
    pub cpu_plimit: u32,
    pub gpu_plimit: u32,
    pub mem_plimit: u32,
}

#[repr(C)]
pub struct KeyData_keyInfo_t {
    pub data_size: u32,
    pub data_type: u32,
    pub data_attributes: u8,
}

#[repr(C)]
pub struct KeyData_t {
    pub key: u32,
    pub vers: KeyData_vers_t,
    pub p_limit_data: KeyData_pLimitData_t,
    pub key_info: KeyData_keyInfo_t,
    pub result: u8,
    pub status: u8,
    pub data8: u8,
    pub data32: u32,
    pub bytes: [i8; 32], // SMCBytes_t
}

#[repr(C)]
pub struct xsw_usage {
    pub xsu_total: u64,
    pub xsu_avail: u64,
    pub xsu_used: u64,
    pub xsu_pagesize: u32,
    pub xsu_encrypted: boolean_t,
}

//pub const HOST_CPU_LOAD_INFO_COUNT: usize = 4;
//pub const HOST_CPU_LOAD_INFO: u32 = 3;
pub const KERN_SUCCESS: u32 = 0;

pub const HW_NCPU: u32 = 3;
pub const CTL_HW: u32 = 6;
pub const CTL_VM: u32 = 2;
pub const VM_SWAPUSAGE: u32 = 5;
pub const PROCESSOR_CPU_LOAD_INFO: u32 = 2;
pub const CPU_STATE_USER: u32 = 0;
pub const CPU_STATE_SYSTEM: u32 = 1;
pub const CPU_STATE_IDLE: u32 = 2;
pub const CPU_STATE_NICE: u32 = 3;
pub const CPU_STATE_MAX: usize = 4;
pub const HW_MEMSIZE: u32 = 24;

//pub const TASK_THREAD_TIMES_INFO: u32 = 3;
//pub const TASK_THREAD_TIMES_INFO_COUNT: u32 = 4;
//pub const TASK_BASIC_INFO_64: u32 = 5;
//pub const TASK_BASIC_INFO_64_COUNT: u32 = 10;
pub const HOST_VM_INFO64: u32 = 4;
pub const HOST_VM_INFO64_COUNT: u32 = 38;

pub const MACH_PORT_NULL: i32 = 0;
pub const KERNEL_INDEX_SMC: i32 = 2;
pub const SMC_CMD_READ_KEYINFO: u8 = 9;
pub const SMC_CMD_READ_BYTES: u8 = 5;

pub const KIO_RETURN_SUCCESS: i32 = 0;
#[allow(non_upper_case_globals)]
pub const kCFStringEncodingMacRoman: CFStringEncoding = 0;
