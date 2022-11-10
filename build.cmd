@echo off
cargo clean -r
cargo build -r
copy %cd%\target\release\fintools.exe C:\Users\plabo\portable\fintools
copy %cd%\ressources\fintools_starter.cmd C:\Users\plabo\portable\fintools
pause