use libc;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Syscall {
    read                = libc::SYS_read                as isize,
    write               = libc::SYS_write               as isize,
    close               = libc::SYS_close               as isize,
    #[cfg(not(target_arch = "aarch64"))]
    stat                = libc::SYS_stat                as isize,
    #[cfg(not(target_arch = "aarch64"))]
    lstat               = libc::SYS_lstat               as isize,
    mmap                = libc::SYS_mmap                as isize,
    mprotect            = libc::SYS_mprotect            as isize,
    munmap              = libc::SYS_munmap              as isize,
    ioctl               = libc::SYS_ioctl               as isize,
    socket              = libc::SYS_socket              as isize,
    connect             = libc::SYS_connect             as isize,
    recvfrom            = libc::SYS_recvfrom            as isize,
    bind                = libc::SYS_bind                as isize,
    clone               = libc::SYS_clone               as isize,
    sigaltstack         = libc::SYS_sigaltstack         as isize,
    futex               = libc::SYS_futex               as isize,
    sched_getaffinity   = libc::SYS_sched_getaffinity   as isize,
    exit_group          = libc::SYS_exit_group          as isize,
    set_robust_list     = libc::SYS_set_robust_list     as isize,
    openat              = libc::SYS_openat              as isize,
    getrandom           = libc::SYS_getrandom           as isize,
    wait4               = libc::SYS_wait4               as isize,
    readlink            = libc::SYS_readlink            as isize,
    mkdir               = libc::SYS_mkdir               as isize,
    unlink              = libc::SYS_unlink              as isize,
    symlink             = libc::SYS_symlink             as isize,
    pipe2               = libc::SYS_pipe2               as isize,
    epoll_create1       = libc::SYS_epoll_create1       as isize,
    epoll_ctl           = libc::SYS_epoll_ctl           as isize,
    epoll_pwait         = libc::SYS_epoll_pwait         as isize,
    listen              = libc::SYS_listen              as isize,
    chmod               = libc::SYS_chmod               as isize,
    accept4             = libc::SYS_accept4             as isize,
    shutdown            = libc::SYS_shutdown            as isize,
    nanosleep           = libc::SYS_nanosleep           as isize,
    sched_yield         = libc::SYS_sched_yield         as isize,
    madvise             = libc::SYS_madvise             as isize,
    exit                = libc::SYS_exit                as isize,
}

impl Syscall {
    #[inline]
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}
