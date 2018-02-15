# tr1pd [![Build Status][travis-img]][travis] [![Crates.io][crates-img]][crates] [![docs.rs][docs-img]][docs]

[travis-img]:   https://travis-ci.org/kpcyrd/tr1pd.svg?branch=master
[travis]:       https://travis-ci.org/kpcyrd/tr1pd
[crates-img]:   https://img.shields.io/crates/v/tr1pd.svg
[crates]:       https://crates.io/crates/tr1pd
[docs-img]:     https://docs.rs/tr1pd/badge.svg
[docs]:         https://docs.rs/tr1pd

**Status: Very unstable, do not use**

tr1pd is a tamper resistant audit log.

## Usage

    # setup your keyring
    tr1pctl init
    # start the tr1pd daemon
    systemctl start tr1pd
    # start a sensor
    ./sensor01 | tr1pctl write &
    # verify your logs
    tr1pctl fsck
    # view the logs of your current session
    tr1pctl ls @..

## Installation

Make sure you have the following dependencies installed:
Debian/Ubuntu: `libsodium-dev libseccomp-dev libzmq3-dev`,
Archlinux: `libsodium libseccomp zeromq`,
Alpine: `make libsodium-dev libseccomp-dev zeromq-dev`,
OpenBSD: `libsodium zeromq`.

    cargo install tr1pd

## Setup

If possible, use your package manager to setup the system ([Archlinux AUR][aur]).
After that you need to add the users that should have access to tr1pctl to the
`tr1pd` group with `usermod -aG tr1pd youruser`.

[aur]: https://aur.archlinux.org/packages/tr1pd/

If no package is available, you can also run a standalone setup (this is also
recommended for development). Edit the paths as needed.

    # standalone configuration (~/.config/tr1pd.toml)

    [daemon]
    socket = "ipc:///home/user/.tr1pd/tr1pd.sock"
    datadir = "/home/user/.tr1pd/"

    pub_key = "/home/user/.tr1pd/pub.key"
    sec_key = "/home/user/.tr1pd/sec.key"

Run `tr1pctl init` to setup the keyring in your homefolder and `tr1pd` in a
seperate terminal. Verify everything is working correctly by executing
`tr1pctl ping`.

## Writing sensors

Sensors can be written in any language using stdio. `tr1pctl write` is a simple
line based interface that writes each line into a block. You can also enable
binary mode with `tr1pctl write -s 65535`. To monitor your auth.log you can
simply write:

    tail -f /var/log/auth.log | tr1pctl write

## The program says block a lot, is this a blockchain?

[No][not a blockchain]. tr1pd uses merkle tree like constructs that are
heavily inspired by bitcoin, but lacks some essential properties to qualify as
a blockchain.

[not a blockchain]: https://gist.github.com/joepie91/e49d2bdc9dfec4adc9da8a8434fd029b

## Trivia

The initial draft for the protocol was designed in 2014 for perimeter
intrustion detection to verify integrity of buildings. Multiple prototypes have
been written in 2017 and the first deployment was on a server located at the
34C3 to ensure integrity inside the congress colocation.

## License

AGPLv3+
