
install-dev:
    cargo install --git=https://github.com/bytecodealliance/wasmtime.git cranelift-tools

# Ref.
# rustup component list
# https://github.com/bytecodealliance/wasmtime/blob/5c8bce70a11657ad6740f56a852f64d147f93494/cranelift/codegen/src/isa/mod.rs#L113C73-L113C80
compile-clif:
    clif-util compile clif-samples/add.clif -o out.o --target x86_64-linux-android
    clif-util compile clif-samples/add.clif -o out.o --target aarch64-apple-darwin
    clif-util compile clif-samples/add.clif -o out.o --target s390x-unknown-linux-gnu
    clif-util compile clif-samples/add.clif -o out.o --target riscv64gc-unknown-none-elf

run-clif:
    clif-util run -v clif-samples/add.clif

run-jit:
    cargo run --bin hi_jit

run-obj:
    cargo run --bin toy

run-hello:
    cargo run --example hello

clif-json:
    cargo install cranelift-serde
    clif-json serialize clif-samples/add.clif


install-cargo-clif:
    rustup install nightly-x86_64-apple-darwin
    rustup component add rustc-codegen-cranelift-preview --toolchain nightly-x86_64-apple-darwin

    git clone https://github.com/rust-lang/rustc_codegen_cranelift
    cd rustc_codegen_cranelift
    cargo build
    ./rustc_codegen_cranelift/target/cargo-clif