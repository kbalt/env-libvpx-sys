@REM Download https://github.com/ShiftMediaProject/libvpx/releases/download/v1.8.2/libvpx_v1.8.2_msvc16.zip
set VPX_VERSION=1.8.2
set VPX_STATIC=1
set VPX_LIB_DIR=%HomeDrive%%HomePath%\unzipped\lib\x64
set VPX_INCLUDE_DIR=%HomeDrive%%HomePath%\unzipped\include
SET VPX_NO_PKG_CONFIG=1

@REM Download llvm from https://releases.llvm.org/download.html
set LIBCLANG_PATH=C:\Program Files\LLVM\bin
cargo build --features=generate
