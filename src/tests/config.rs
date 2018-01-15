use config::{Config, DaemonConfig, SecurityConfig};


#[test]
fn parse_empty_config() {
    let data = r#"

    "#;

    let config = Config::parse(&data).unwrap();
    assert_eq!(config, Config::default());
}

#[test]
fn parse_config() {
    let data = r#"
    [daemon]
    socket = "ipc:///run/tr1pd/tr1pd.sock"
    datadir = "/var/lib/tr1pd"

    pub_key = "/etc/tr1pd/pub.key"
    sec_key = "/etc/tr1pd/sec.key"

    [security]
    strict_chroot = true
    "#;

    let config = Config::parse(&data).unwrap();
    assert_eq!(config, Config {
        daemon: DaemonConfig {
            socket: Some("ipc:///run/tr1pd/tr1pd.sock".into()),
            datadir: Some("/var/lib/tr1pd".into()),

            pub_key: Some("/etc/tr1pd/pub.key".into()),
            sec_key: Some("/etc/tr1pd/sec.key".into()),
        },
        security: SecurityConfig {
            strict_chroot: true,
        },
    });
}
