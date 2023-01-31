@echo off
start /b cargo build --target i686-pc-windows-msvc --lib
start /b cargo build --target x86_64-pc-windows-msvc --bin modrunner
