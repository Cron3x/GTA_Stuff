@echo off

cd installer\
cargo build --release

cd ..\program\
cargo build --release

cd ..\
mkdir "release"
copy installer\target\release\gta_stuff_installer.exe .
copy program\release\release\gta_stuff.exe .