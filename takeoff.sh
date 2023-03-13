#!/bin/bash
OLD=${pwd}
cd "$HOME/.config/nvim/takeoff/target/release"
cargo run --release -- $@
cd $OLD
