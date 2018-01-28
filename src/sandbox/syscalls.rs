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
    fstat               = libc::SYS_fstat               as isize,
    newfstatat          = libc::SYS_newfstatat          as isize,
    getdents            = libc::SYS_getdents            as isize,
    getpid              = libc::SYS_getpid              as isize,
    getuid              = libc::SYS_getuid              as isize,
    readv               = libc::SYS_readv               as isize,
    lseek               = libc::SYS_lseek               as isize,
    eventfd2            = libc::SYS_eventfd2            as isize,
    sched_getparam      = libc::SYS_sched_getparam      as isize,
    sched_getscheduler  = libc::SYS_sched_getscheduler  as isize,
    sched_setscheduler  = libc::SYS_sched_setscheduler  as isize,
    getsockname         = libc::SYS_getsockname         as isize,
    getsockopt          = libc::SYS_getsockopt          as isize,
    getpeername         = libc::SYS_getpeername         as isize,
    poll                = libc::SYS_poll                as isize,
    sendto              = libc::SYS_sendto              as isize,
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
    #[cfg(not(target_arch = "aarch64"))]
    open                = libc::SYS_open                as isize,
    openat              = libc::SYS_openat              as isize,
    getrandom           = libc::SYS_getrandom           as isize,
    wait4               = libc::SYS_wait4               as isize,
    #[cfg(not(target_arch = "aarch64"))]
    readlink            = libc::SYS_readlink            as isize,
    readlinkat          = libc::SYS_readlinkat          as isize,
    #[cfg(not(target_arch = "aarch64"))]
    mkdir               = libc::SYS_mkdir               as isize,
    mkdirat             = libc::SYS_mkdirat             as isize,
    #[cfg(not(target_arch = "aarch64"))]
    unlink              = libc::SYS_unlink              as isize,
    unlinkat            = libc::SYS_unlinkat            as isize,
    #[cfg(not(target_arch = "aarch64"))]
    symlink             = libc::SYS_symlink             as isize,
    symlinkat           = libc::SYS_symlinkat           as isize,
    pipe2               = libc::SYS_pipe2               as isize,
    epoll_create1       = libc::SYS_epoll_create1       as isize,
    epoll_ctl           = libc::SYS_epoll_ctl           as isize,
    epoll_pwait         = libc::SYS_epoll_pwait         as isize,
    #[cfg(not(target_arch = "aarch64"))]
    epoll_wait          = libc::SYS_epoll_wait          as isize,
    listen              = libc::SYS_listen              as isize,
    #[cfg(not(target_arch = "aarch64"))]
    chmod               = libc::SYS_chmod               as isize,
    fchmodat            = libc::SYS_fchmodat            as isize,
    accept4             = libc::SYS_accept4             as isize,
    shutdown            = libc::SYS_shutdown            as isize,
    nanosleep           = libc::SYS_nanosleep           as isize,
    sched_yield         = libc::SYS_sched_yield         as isize,
    madvise             = libc::SYS_madvise             as isize,
    exit                = libc::SYS_exit                as isize,
    prctl               = libc::SYS_prctl               as isize,
    seccomp             = libc::SYS_seccomp             as isize,
    capget              = libc::SYS_capget              as isize,
    capset              = libc::SYS_capset              as isize,
    chroot              = libc::SYS_chroot              as isize,
    chdir               = libc::SYS_chdir               as isize,
    fcntl               = libc::SYS_fcntl               as isize,
    brk                 = libc::SYS_brk                 as isize,
    rt_sigprocmask      = libc::SYS_rt_sigprocmask      as isize,
}

impl Syscall {
    #[inline]
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}
