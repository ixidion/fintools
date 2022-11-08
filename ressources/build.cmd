@echo off
cd ..
cargo build -r
copy target\release\fintools.exe C:\Users\plabo\portable\fintools
copy ressources\fintools_starter.ps1 C:\Users\plabo\portable\fintools
pause