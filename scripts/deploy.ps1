# Build Application
cargo build --release --target x86_64-pc-windows-msvc --target-dir ./build

# Move Executable
if (Test-Path -Path ./bin) {
    rm -r -fo ./bin
}

mkdir ./bin
move ./build/x86_64-pc-windows-msvc/release/proext.exe ./bin

# Save Hash
$(CertUtil -hashfile ./bin/proext.exe MD5)[1] -replace " ","" > ./bin/hash.txt