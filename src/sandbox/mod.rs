use config::Config;

mod errors {
    #[cfg(target_os="linux")]
    use sandbox::capabilities;
    use sandbox::chroot;
    #[cfg(target_os="openbsd")]
    use sandbox::pledge;
    #[cfg(target_os="linux")]
    use sandbox::seccomp;

    error_chain! {
        links {
            Capabilities(capabilities::Error, capabilities::ErrorKind)  #[cfg(target_os="linux")];
            Chroot(chroot::Error, chroot::ErrorKind);
            Pledge(pledge::Error, pledge::ErrorKind) #[cfg(target_os="openbsd")];
            Seccomp(seccomp::Error, seccomp::ErrorKind) #[cfg(target_os="linux")];
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};

#[cfg(target_os="linux")]
pub mod capabilities;
pub mod chroot;
#[cfg(target_os="openbsd")]
pub mod pledge;
#[cfg(target_os="linux")]
pub mod seccomp;
#[cfg(target_os="linux")]
pub mod syscalls;


pub fn activate_stage1() -> Result<()> {
    #[cfg(target_os="linux")]
    seccomp::activate_stage1()?;

    #[cfg(target_os="openbsd")]
    pledge::activate_stage1()?;

    info!("stage 1/2 is active");

    Ok(())
}

pub fn activate_stage2(mut config: &mut Config) -> Result<()> {
    chroot::lock_to_datadir(&mut config)?;

    #[cfg(target_os="linux")]
    capabilities::drop()?;

    #[cfg(target_os="linux")]
    seccomp::activate_tr1pd_stage2()?;

    #[cfg(target_os="openbsd")]
    pledge::activate_tr1pd_stage2()?;

    info!("stage 2/2 is active");

    Ok(())
}
