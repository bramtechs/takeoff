set root=%LOCALAPPDATA%\nvim\takeoff
set reldir=%root%\target\release
set exe=%reldir%\takeoff.exe

if not exist %exe% (
    pushd %reldir%
    cargo build --release
    popd
)
call "%exe%" %*

