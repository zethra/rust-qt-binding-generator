extern crate clap;
extern crate rust_qt_binding_generator;
use clap::{App, Arg};
use rust_qt_binding_generator::*;

fn main() {
    let matches = App::new("rust_qt_binding_generator")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Generates bindings between Qt and Rust")
        .arg(
            Arg::with_name("overwrite-implementation")
                .long("overwrite-implementation")
                .help("Overwrite existing implementation."),
        )
        .arg(
            Arg::with_name("edition")
                .long("edition")
                .takes_value(true)
                .help("Rust edition (default 2015)")
        )
        .arg(
            Arg::with_name("config")
                .multiple(true)
                .required(true)
                .takes_value(true)
                .help("Configuration file(s)"),
        )
        .get_matches();

    let overwrite_implementation = matches.is_present("overwrite-implementation");
    let edition = matches.value_of("edition");
    for config in matches.values_of("config").unwrap() {
        if let Err(e) = generate_bindings_from_config_file(config, overwrite_implementation, edition) {
            eprintln!("{}", e);
            ::std::process::exit(1);
        }
    }
}
