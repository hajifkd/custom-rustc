#![feature(rustc_private)]

extern crate rustc_driver;

use rustc_driver::{run_compiler, RustcDefaultCalls};
use std::env;

fn main() {
    let args = env::args_os()
        .map(|arg| arg.into_string().unwrap())
        .collect::<Vec<_>>();
    run_compiler(&args, &mut RustcDefaultCalls, None, None);
}
