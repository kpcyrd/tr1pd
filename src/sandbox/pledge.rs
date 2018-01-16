use pledge::{pledge, Promise, ToPromiseString};

mod errors {
    use pledge;

    error_chain! {
        foreign_links {
            Pledge(pledge::Error);
        }
    }
}
pub use self::errors::{Result, Error, ErrorKind};


#[inline]
pub fn activate_stage1() -> Result<()> {
    info!("calling pledge");
    pledge![Stdio, RPath, WPath, CPath, Dns, Unix, Fattr, Inet]?;

    info!("stage 1/2 is active");
    Ok(())
}

#[inline]
pub fn activate_tr1pd_stage2() -> Result<()> {
    info!("calling pledge");
    pledge![Stdio, RPath, WPath, CPath, Inet]?;

    info!("stage 2/2 is active");
    Ok(())
}
