#[macro_use]
extern crate clap;

mod app;
mod config;
mod context;
mod init;
mod module;
mod modules;
mod print;
mod segment;
mod utils;

fn main() {
    pretty_env_logger::init();

    let matches = app::app().get_matches();

    match matches.subcommand() {
        ("init", Some(sub_m)) => {
            let shell_name = sub_m.value_of("shell").expect("Shell name missing.");
            if sub_m.is_present("print_full_init") {
                init::init_main(shell_name);
            } else {
                init::init_stub(shell_name);
            }
        }
        ("prompt", Some(args)) => print::prompt(args.clone()),
        ("module", Some(args)) => {
            let module_name = args.value_of("name").expect("Module name missing.");
            print::module(module_name, args.clone());
        }
        _ => {}
    }
}
