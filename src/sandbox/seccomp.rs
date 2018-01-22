use seccomp_sys::*;

use sandbox::syscalls::Syscall;

mod errors {
    error_chain! {
        errors {
            FFI
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};


pub struct Context {
    ctx: *mut scmp_filter_ctx,
}

impl Context {
    fn init() -> Result<Context> {
        let ctx = unsafe { seccomp_init(SCMP_ACT_KILL) };

        if ctx.is_null() {
            return Err(ErrorKind::FFI.into());
        }

        Ok(Context {
            ctx,
        })
    }

    fn allow_syscall(&mut self, syscall: Syscall) -> Result<()> {
        debug!("seccomp: allowing syscall={:?}", syscall);
        let ret = unsafe { seccomp_rule_add(self.ctx, SCMP_ACT_ALLOW, syscall.as_i32(), 0) };

        if ret != 0 {
            Err(ErrorKind::FFI.into())
        } else {
            Ok(())
        }
    }

    fn load(&self) -> Result<()> {
        let ret = unsafe { seccomp_load(self.ctx) };

        if ret != 0 {
            Err(ErrorKind::FFI.into())
        } else {
            Ok(())
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            seccomp_release(self.ctx)
        };
    }
}

pub fn activate_stage1() -> Result<()> {
    let mut ctx = Context::init()?;

    ctx.allow_syscall(Syscall::read)?;
    ctx.allow_syscall(Syscall::write)?;
    ctx.allow_syscall(Syscall::mmap)?;
    ctx.allow_syscall(Syscall::mprotect)?;
    ctx.allow_syscall(Syscall::getrandom)?;
    ctx.allow_syscall(Syscall::futex)?;
    ctx.allow_syscall(Syscall::openat)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::open)?;
    ctx.allow_syscall(Syscall::ioctl)?;
    ctx.allow_syscall(Syscall::close)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::readlink)?;
    ctx.allow_syscall(Syscall::readlinkat)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::mkdir)?;
    ctx.allow_syscall(Syscall::mkdirat)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::lstat)?;
    ctx.allow_syscall(Syscall::fstat)?;
    ctx.allow_syscall(Syscall::newfstatat)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::unlink)?;
    ctx.allow_syscall(Syscall::unlinkat)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::symlink)?;
    ctx.allow_syscall(Syscall::symlinkat)?;
    ctx.allow_syscall(Syscall::getdents)?;
    ctx.allow_syscall(Syscall::getpid)?;
    ctx.allow_syscall(Syscall::eventfd2)?;
    ctx.allow_syscall(Syscall::sched_getparam)?;
    ctx.allow_syscall(Syscall::sched_getscheduler)?;
    ctx.allow_syscall(Syscall::sched_setscheduler)?;
    ctx.allow_syscall(Syscall::poll)?;
    ctx.allow_syscall(Syscall::getsockname)?;
    ctx.allow_syscall(Syscall::getsockopt)?;
    ctx.allow_syscall(Syscall::getpeername)?;
    ctx.allow_syscall(Syscall::sendto)?;
    ctx.allow_syscall(Syscall::clone)?;
    ctx.allow_syscall(Syscall::set_robust_list)?;
    ctx.allow_syscall(Syscall::sigaltstack)?;
    ctx.allow_syscall(Syscall::munmap)?;
    ctx.allow_syscall(Syscall::sched_getaffinity)?;
    ctx.allow_syscall(Syscall::pipe2)?;
    ctx.allow_syscall(Syscall::epoll_create1)?;
    ctx.allow_syscall(Syscall::epoll_ctl)?;
    ctx.allow_syscall(Syscall::epoll_pwait)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::epoll_wait)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::stat)?;
    ctx.allow_syscall(Syscall::socket)?;
    ctx.allow_syscall(Syscall::bind)?;
    ctx.allow_syscall(Syscall::listen)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::chmod)?;
    ctx.allow_syscall(Syscall::fchmodat)?;
    ctx.allow_syscall(Syscall::accept4)?;
    ctx.allow_syscall(Syscall::recvfrom)?;
    ctx.allow_syscall(Syscall::shutdown)?;
    ctx.allow_syscall(Syscall::connect)?;
    ctx.allow_syscall(Syscall::nanosleep)?;
    ctx.allow_syscall(Syscall::sched_yield)?;
    ctx.allow_syscall(Syscall::madvise)?;
    ctx.allow_syscall(Syscall::exit_group)?;
    ctx.allow_syscall(Syscall::exit)?;
    ctx.allow_syscall(Syscall::wait4)?;
    ctx.allow_syscall(Syscall::fcntl)?;
    ctx.allow_syscall(Syscall::brk)?;
    ctx.allow_syscall(Syscall::rt_sigprocmask)?;
    ctx.allow_syscall(Syscall::prctl)?; // needed for stage2
    ctx.allow_syscall(Syscall::seccomp)?; // needed for stage2
    ctx.allow_syscall(Syscall::capget)?; // needed for stage2
    ctx.allow_syscall(Syscall::capset)?; // needed for stage2
    ctx.allow_syscall(Syscall::chroot)?; // needed for stage2
    ctx.allow_syscall(Syscall::chdir)?; // needed for stage2

    ctx.load()?;

    info!("stage 1/1 is active");

    Ok(())
}

pub fn activate_tr1pd_stage2() -> Result<()> {
    let mut ctx = Context::init()?;

    ctx.allow_syscall(Syscall::read)?;
    ctx.allow_syscall(Syscall::write)?;
    ctx.allow_syscall(Syscall::mmap)?;
    ctx.allow_syscall(Syscall::mprotect)?;
    ctx.allow_syscall(Syscall::getrandom)?;
    ctx.allow_syscall(Syscall::futex)?;
    ctx.allow_syscall(Syscall::openat)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::open)?;
    ctx.allow_syscall(Syscall::ioctl)?;
    ctx.allow_syscall(Syscall::close)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::readlink)?;
    ctx.allow_syscall(Syscall::readlinkat)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::mkdir)?;
    ctx.allow_syscall(Syscall::mkdirat)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::lstat)?;
    ctx.allow_syscall(Syscall::newfstatat)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::unlink)?;
    ctx.allow_syscall(Syscall::unlinkat)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::symlink)?;
    ctx.allow_syscall(Syscall::symlinkat)?;
    ctx.allow_syscall(Syscall::sched_getparam)?;
    ctx.allow_syscall(Syscall::sched_getscheduler)?;
    ctx.allow_syscall(Syscall::sched_setscheduler)?;
    ctx.allow_syscall(Syscall::getpeername)?;
    ctx.allow_syscall(Syscall::eventfd2)?;
    ctx.allow_syscall(Syscall::getpid)?;
    ctx.allow_syscall(Syscall::poll)?;
    ctx.allow_syscall(Syscall::sendto)?;
    ctx.allow_syscall(Syscall::clone)?;
    ctx.allow_syscall(Syscall::set_robust_list)?;
    ctx.allow_syscall(Syscall::sigaltstack)?;
    ctx.allow_syscall(Syscall::munmap)?;
    ctx.allow_syscall(Syscall::sched_getaffinity)?;
    ctx.allow_syscall(Syscall::pipe2)?;
    ctx.allow_syscall(Syscall::epoll_create1)?;
    ctx.allow_syscall(Syscall::epoll_ctl)?;
    ctx.allow_syscall(Syscall::epoll_pwait)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::epoll_wait)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::stat)?;
    ctx.allow_syscall(Syscall::socket)?;
    ctx.allow_syscall(Syscall::bind)?;
    ctx.allow_syscall(Syscall::listen)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::chmod)?;
    ctx.allow_syscall(Syscall::fchmodat)?;
    ctx.allow_syscall(Syscall::accept4)?;
    ctx.allow_syscall(Syscall::recvfrom)?;
    ctx.allow_syscall(Syscall::shutdown)?;
    ctx.allow_syscall(Syscall::connect)?;
    ctx.allow_syscall(Syscall::nanosleep)?;
    ctx.allow_syscall(Syscall::sched_yield)?;
    ctx.allow_syscall(Syscall::madvise)?;
    ctx.allow_syscall(Syscall::exit_group)?;
    ctx.allow_syscall(Syscall::exit)?;
    ctx.allow_syscall(Syscall::wait4)?;
    ctx.allow_syscall(Syscall::fcntl)?;
    ctx.allow_syscall(Syscall::brk)?;
    // ctx.allow_syscall(Syscall::prctl)?; // needed for stage2
    // ctx.allow_syscall(Syscall::seccomp)?; // needed for stage2
    // ctx.allow_syscall(Syscall::capget)?; // needed for stage2 TODO
    // ctx.allow_syscall(Syscall::capset)?; // needed for stage2 TODO
    // ctx.allow_syscall(Syscall::chroot)?; // needed for stage2 TODO
    // ctx.allow_syscall(Syscall::chdir)?; // needed for stage2 TODO

    ctx.load()?;

    info!("stage 2/2 is active");

    Ok(())
}
