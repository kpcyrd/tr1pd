mod errors {
    use sandbox::seccomp;

    error_chain! {
        links {
            Seccomp(seccomp::Error, seccomp::ErrorKind);
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};


pub mod seccomp;
pub mod syscalls;

pub fn activate_stage1() -> Result<()> {
    #[cfg(target_os="linux")]
    seccomp::activate_stage1()?;

    info!("stage 1/2 is active");

    Ok(())
}
