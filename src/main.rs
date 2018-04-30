#![feature(rustc_private)]

extern crate getopts;
extern crate rustc;
extern crate rustc_driver;

use rustc::session::config::Input;
use rustc::session::Session;
use rustc_driver::driver::{CompileController, CompileState};
use rustc_driver::{run_compiler, Compilation, CompilerCalls};

use std::env;

struct MyRustcCall;

impl<'a> CompilerCalls<'a> for MyRustcCall {
    fn build_controller(
        &mut self,
        _session: &Session,
        _opts: &getopts::Matches,
    ) -> CompileController<'a> {
        let mut controller = CompileController::basic();
        controller.after_parse.stop = Compilation::Stop;
        controller.after_parse.callback = Box::new(|state: &mut CompileState| {
            println!("Hello, my compiler!",);

            // take a look at `input`
            match state.input {
                Input::File(ref p) => {
                    println!("{:?}", p);
                }
                Input::Str { name, input } => {
                    println!("name: {}, input: {}", name, input);
                }
            }

            println!("{:?}", state.krate);
        });

        controller
    }
}

fn main() {
    let args = env::args_os()
        .map(|arg| arg.into_string().unwrap())
        .collect::<Vec<_>>();
    run_compiler(&args, &mut MyRustcCall, None, None);
}
