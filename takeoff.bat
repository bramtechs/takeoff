set exe=target/release/takeoff.exe
if not exist %exe% (
    cargo build --release
)
call "%exe%" %*

