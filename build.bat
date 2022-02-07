@echo off

cd ..\program\
cargo clean
cargo build --release

cd ..\

mkdir "_build"

copy program\target\release\gta_stuff.exe .\_build\
Xcopy /E program\scripts\ .\_build\scripts\

exit