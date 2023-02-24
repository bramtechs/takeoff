@echo off

set root=%LOCALAPPDATA%\nvim\takeoff
set reldir=%root%\target\release
set exe=%reldir%\takeoff.exe

if not exist %exe% (
    pushd %root%
    cargo build --release
    popd
)
call "%exe%" %*
