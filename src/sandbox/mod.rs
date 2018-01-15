use config::Config;

mod errors {
    use sandbox::seccomp;
    use sandbox::chroot;

    error_chain! {
        links {
            Seccomp(seccomp::Error, seccomp::ErrorKind);
            Chroot(chroot::Error, chroot::ErrorKind);
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};

pub mod chroot;
pub mod seccomp;
pub mod syscalls;


pub fn activate_stage1() -> Result<()> {
    #[cfg(target_os="linux")]
    seccomp::activate_stage1()?;

    info!("stage 1/2 is active");

    Ok(())
}

pub fn activate_stage2(config: &mut Config) -> Result<()> {
    if chroot::can_chroot()? {
        {
            let target = config.datadir();
            debug!("chroot: -> {:?}", target);
            chroot::chroot(target)?;
        }
        config.set_datadir(Some("/"));

        chroot::drop_caps()?;
    } else if config.security.strict_chroot {
        panic!("strict-chroot is set and process didn't chroot");
    }

    #[cfg(target_os="linux")]
    seccomp::activate_tr1pd_stage2()?;

    Ok(())
}
