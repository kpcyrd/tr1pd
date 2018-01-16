use libc;
#[cfg(not(target_os="linux"))]
use users;

#[cfg(target_os="linux")]
use sandbox::capabilities;
use config::Config;

use std::env;
use std::ffi::CString;

mod errors {
    #[cfg(target_os="linux")]
    use sandbox::capabilities;

    error_chain! {
        errors {
            FFI
        }
        links {
            Caps(capabilities::Error, capabilities::ErrorKind) #[cfg(target_os="linux")];
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};


#[cfg(target_os="linux")]
#[inline]
pub fn can_chroot() -> Result<bool> {
    let perm_chroot = capabilities::can_chroot()?;
    Ok(perm_chroot)
}

#[cfg(not(target_os="linux"))]
#[inline]
pub fn can_chroot() -> Result<bool> {
    let is_root = users::get_effective_uid() == 0;
    Ok(is_root)
}

#[inline]
pub fn lock_to_datadir(config: &mut Config) -> Result<()> {
    if can_chroot()? {
        {
            let target = config.datadir();
            debug!("chroot: -> {:?}", target);
            chroot(target)?;
        }
        config.set_datadir(Some("/"));

        // XXX: it's currently not recommended to use chroot
        // on a platform that isn't linux since we don't have
        // capabilities(7) there and we don't have setuid code yet.
    } else if config.security.strict_chroot {
        panic!("strict-chroot is set and process didn't chroot");
    }

    Ok(())
}

#[inline]
pub fn chroot(path: &str) -> Result<()> {
    let path = CString::new(path).unwrap();
    let ret = unsafe { libc::chroot(path.as_ptr()) };

    if ret != 0 {
        Err(ErrorKind::FFI.into())
    } else {
        match env::set_current_dir("/") {
            Ok(_) => Ok(()),
            Err(_) => Err(ErrorKind::FFI.into()),
        }
    }
}
