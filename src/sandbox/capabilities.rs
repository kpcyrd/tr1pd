use caps::{self, CapSet, Capability};

mod errors {
    use caps;

    error_chain! {
        foreign_links {
            Caps(caps::errors::Error);
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};


#[inline]
pub fn log_permitted_caps() -> Result<()> {
    let cur = caps::read(None, CapSet::Permitted)?;
    debug!("caps: permitted caps: {:?}.", cur);
    Ok(())
}

#[inline]
pub fn can_chroot() -> Result<bool> {
    log_permitted_caps()?;

    let perm_chroot = caps::has_cap(None, CapSet::Permitted, Capability::CAP_SYS_CHROOT)?;
    info!("caps: can chroot: {:?}", perm_chroot);

    Ok(perm_chroot)
}

#[inline]
pub fn drop() -> Result<()> {
    caps::clear(None, CapSet::Permitted)?;
    info!("caps: permitted caps cleared");

    log_permitted_caps()?;

    Ok(())
}
