@echo off
cargo clean -r
cargo build -r
copy %cd%\target\release\fintools.exe C:\Users\plabo\portable\fintools
pause