root=`rustc --print sysroot`
cargo run -- -L ${root}/lib/rustlib/x86_64-unknown-linux-gnu/lib/ $1