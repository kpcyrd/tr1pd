use config::Config;

mod errors {
    #[cfg(target_os="linux")]
    use sandbox::seccomp;
    use sandbox::chroot;

    #[cfg(target_os="linux")]
    error_chain! {
        links {
            Seccomp(seccomp::Error, seccomp::ErrorKind);
            Chroot(chroot::Error, chroot::ErrorKind);
        }
    }

    #[cfg(not(target_os="linux"))]
    error_chain! {
        links {
            Chroot(chroot::Error, chroot::ErrorKind);
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};

pub mod chroot;
#[cfg(target_os="linux")]
pub mod seccomp;
#[cfg(target_os="linux")]
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

        // XXX: it's currently not recommended to use chroot
        // on a platform that isn't linux since we don't have
        // capabilities(7) there and there's no setuid code yet.

        #[cfg(target_os="linux")]
        chroot::drop_caps()?;
    } else if config.security.strict_chroot {
        panic!("strict-chroot is set and process didn't chroot");
    }

    #[cfg(target_os="linux")]
    seccomp::activate_tr1pd_stage2()?;

    Ok(())
}
