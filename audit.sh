#!/bin/sh
(journalctl -f -n 250 \
	-t systemd-logind \
	-t kernel \
	-t systemd \
	-t sshd \
	-t login \
	-t systemd-udevd \
	-t sudo
) | /root/tr1pd/target/release/tr1pd
