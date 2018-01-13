use clap::Arg;

#[inline]
pub fn socket() -> Arg<'static, 'static> {
    Arg::with_name("socket")
        .short("S")
        .long("socket")
        .takes_value(true)
        .env("TR1PD_SOCKET")
}

#[inline]
pub fn data_dir() -> Arg<'static, 'static> {
    Arg::with_name("data-dir")
        .short("D")
        .long("data-dir")
        .takes_value(true)
        .env("TR1PD_DATADIR")
}
