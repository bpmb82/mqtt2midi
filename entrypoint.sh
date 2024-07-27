#!/bin/sh

cargo build --release --target-dir /mqtt2midi/target
echo "Release was built to $DIR/target/"
chown -R ${USERID}:${USERID} /mqtt2midi/target