@echo off
cargo rustc --release -- --emit=asm
cargo run --release
pause
exit
