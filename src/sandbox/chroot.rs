use libc;
#[cfg(not(target_os="linux"))]
use users;
#[cfg(target_os="linux")]
use caps::{self, CapSet, Capability};

use std::env;
use std::ffi::CString;

mod errors {
    #[cfg(target_os="linux")]
    use caps;

    error_chain! {
        errors {
            FFI
        }
        foreign_links {
            Caps(caps::errors::Error) #[cfg(target_os="linux")];
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};


#[cfg(target_os="linux")]
#[inline]
pub fn log_permitted_caps() -> Result<()> {
    let cur = caps::read(None, CapSet::Permitted)?;
    debug!("caps: permitted caps: {:?}.", cur);
    Ok(())
}

#[cfg(target_os="linux")]
#[inline]
pub fn can_chroot() -> Result<bool> {
    log_permitted_caps()?;

    let perm_chroot = caps::has_cap(None, CapSet::Permitted, Capability::CAP_SYS_CHROOT)?;
    info!("caps: can chroot: {:?}", perm_chroot);

    Ok(perm_chroot)
}

#[cfg(not(target_os="linux"))]
#[inline]
pub fn can_chroot() -> Result<bool> {
    let is_root = users::get_effective_uid() == 0;
    Ok(is_root)
}

#[cfg(target_os="linux")]
#[inline]
pub fn drop_caps() -> Result<()> {
    caps::clear(None, CapSet::Permitted)?;
    info!("caps: permitted caps cleared");

    log_permitted_caps()?;

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
