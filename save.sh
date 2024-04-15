#!/usr/bin/env bash

set -e
set -x

today=$(date +"%Y-%m-%d")
tarball=releves-$today.tar.gz

tar cvzf $tarball releves
dedix-put $tarball
