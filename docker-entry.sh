#!/bin/sh
set -e
tr1pctl init
exec "$@"
