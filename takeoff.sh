#!/bin/bash
DIR="$HOME/.config/nvim/takeoff/target/release"
EXE="takeoff"
if ! [ -f "$DIR/$EXE" ]; then
    OLD=${pwd}
    cd $DIR
    cargo build --release
    cd $OLD
fi

exec $DIR/$EXE $@
