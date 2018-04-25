root=`rustc --print sysroot`
target=`rustup target list|grep default | awk '{print $1}'`
cargo run -- -L "${root}/lib/rustlib/${target}/lib/" $1