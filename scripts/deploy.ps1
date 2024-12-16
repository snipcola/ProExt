# Build Application
cargo build --release --target x86_64-pc-windows-msvc --target-dir ./build

# Remove Existing Executable
if (Test-Path -Path ./bin) {
    rm -r -fo ./bin
}

# Move Executable
mkdir ./bin
move ./build/x86_64-pc-windows-msvc/release/proext.exe ./bin