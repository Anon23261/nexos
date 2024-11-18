@echo off
echo Setting up development environment for NexOS...

:: Create directories
mkdir tools
cd tools

:: Download Rust installer
echo Downloading Rust installer...
powershell -Command "Invoke-WebRequest -Uri 'https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe' -OutFile 'rustup-init.exe'"

:: Run Rust installer
echo Installing Rust...
rustup-init.exe -y --default-toolchain nightly

:: Download Visual Studio Build Tools
echo Downloading Visual Studio Build Tools...
powershell -Command "Invoke-WebRequest -Uri 'https://aka.ms/vs/17/release/vs_buildtools.exe' -OutFile 'vs_buildtools.exe'"

:: Install Visual Studio Build Tools
echo Installing Visual Studio Build Tools...
vs_buildtools.exe --quiet --wait --norestart --nocache --installPath "%ProgramFiles(x86)%\Microsoft Visual Studio\2022\BuildTools" --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows10SDK.19041

:: Download QEMU
echo Downloading QEMU...
powershell -Command "Invoke-WebRequest -Uri 'https://qemu.weilnetz.de/w64/2023/qemu-w64-setup-20231211.exe' -OutFile 'qemu-setup.exe'"

echo Setup files downloaded!
echo.
echo Please follow these steps:
echo 1. Run rustup-init.exe if it didn't run automatically
echo 2. Run vs_buildtools.exe and select "Desktop development with C++"
echo 3. Run qemu-setup.exe and follow the installation wizard
echo.
echo After installing, restart your computer and we can continue with the OS development.
pause
