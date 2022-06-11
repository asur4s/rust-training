use std::os::unix::io::{AsRawFd, RawFd};
use std::{io, mem, ptr};

use evdev::Device;

pub fn is_readable(device: &mut Device) -> bool {
    // TODO：可能需要由调用者准备
    let mut fd_set = FdSet::new();
    fd_set.set(device.as_raw_fd());

    // 判断文件描述符是否可以读
    let result = match unsafe {
        libc::select(
            device.as_raw_fd() + 1,
            to_fdset_ptr(Some(&mut fd_set)),
            to_fdset_ptr(None),
            to_fdset_ptr(None),
            to_ptr::<libc::timeval>(None) as *mut libc::timeval,
        )
    } {
        -1 => Err(io::Error::last_os_error()),
        res => Ok(res as usize),
    }
    .unwrap();

    // TODO：为什么要将文件描述符加入到 FdSet 中。
    result == 1 && fd_set.is_set(device.as_raw_fd())
}

// 获取句柄集合的指针
fn to_fdset_ptr(opt: Option<&mut FdSet>) -> *mut libc::fd_set {
    match opt {
        None => ptr::null_mut(),
        Some(&mut FdSet((ref mut raw_fd_set))) => raw_fd_set,
    }
}

fn to_ptr<T>(opt: Option<&T>) -> *const T {
    match opt {
        None => ptr::null::<T>(),
        Some(p) => p,
    }
}

// 句柄集合
struct FdSet(libc::fd_set);

impl FdSet {
    fn new() -> FdSet {
        unsafe {
            // 创建文件句柄集合
            let mut raw_fd_set = mem::MaybeUninit::<libc::fd_set>::uninit();
            // FD_ZERO 清空集合
            libc::FD_ZERO(raw_fd_set.as_mut_ptr());
            // 初始化
            FdSet(raw_fd_set.assume_init())
        }
    }

    fn set(&mut self, fd: RawFd) {
        unsafe {
            // 往句柄集合里面加入句柄。
            libc::FD_SET(fd, &mut self.0)
        }
    }

    // 判断是否已经加入到句柄集合。
    fn is_set(&mut self, fd: RawFd) -> bool {
        unsafe { libc::FD_ISSET(fd, &mut self.0) }
    }
}
