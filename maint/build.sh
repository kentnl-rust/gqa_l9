export CC="x86_64-pc-linux-gnu-gcc"
#export RUSTFLAGS="-Clinker=x86_64-pc-linux-gnu-gcc"
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C linker=x86_64-pc-linux-gnu-gcc"
cargo build "$@"
