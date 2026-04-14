//! Process management syscalls
use crate::config::MAX_SYSCALL_NUM;
use crate::task::{change_program_brk, current_user_token, exit_current_and_run_next, mmap, munmap, suspend_current_and_run_next};
use crate::task::get_syscall_count;
use crate::mm::{PTEFlags, PageTable, VirtAddr};
use crate::timer::get_time_us;
//use crate::config::MAX_SYSCALL_NUM;
//use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let sec = us / 1_000_000;
    let usec = us % 1_000_000;
    let time_val = TimeVal { sec, usec };
    let ts_usize = ts as *const TimeVal as usize;
    for i in 0..core::mem::size_of::<TimeVal>() {
        let addr = ts_usize + i;
        let byte = unsafe { (&time_val as *const TimeVal as *const u8).add(i).read() };
        if write_user_u8(addr, byte).is_none() {
            return -1;
        }
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => read_user_u8(id).map_or(-1, |data| data as isize),
        1 => {
            if write_user_u8(id, data as u8).is_some() {
                0
            } else {
                -1
            }
        }
        2 => {
            if id < MAX_SYSCALL_NUM {
                get_syscall_count(id) as isize
            } else {
                -1
            }
        }
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");
    mmap(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    munmap(start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
///
fn read_user_u8(addr: usize) -> Option<u8> {
    let page_table = PageTable::from_token(current_user_token());
    let va = VirtAddr::from(addr);
    let pte = page_table.translate(va.floor())?;
    let flags = pte.flags();
    if !flags.contains(PTEFlags::U) || !flags.contains(PTEFlags::R) {
        return None;
    }
    Some(pte.ppn().get_bytes_array()[va.page_offset()])
}
///
fn write_user_u8(addr: usize, byte: u8) -> Option<()> {
    let page_table = PageTable::from_token(current_user_token());
    let va = VirtAddr::from(addr);
    let pte = page_table.translate(va.floor())?;
    let flags = pte.flags();
    if !flags.contains(PTEFlags::U) || !flags.contains(PTEFlags::W) {
        return None;
    }
    pte.ppn().get_bytes_array()[va.page_offset()] = byte;
    Some(())
}