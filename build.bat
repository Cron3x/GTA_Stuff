@echo off

cd installer\
cargo build --release

cd ..\program\
cargo build --release

cd ..\
mkdir "release"
mkdir "release\program"
copy installer\target\release\gta_stuff_installer.exe release\
copy program\target\release\gta_stuff.exe release\program
Xcopy /E program\scripts\ release\program\scripts\

7z a release\gta_stuff.zip .\program

timeout 3
exit