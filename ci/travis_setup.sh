#!/bin/sh
set -ex
apt-get update -q
# update docker
apt-get -y -o Dpkg::Options::="--force-confnew" install docker-ce
