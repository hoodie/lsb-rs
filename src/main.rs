#![allow(unused_imports)]
#![allow(dead_code)]
extern crate image;
extern crate bit_vec;
#[macro_use]
extern crate clap;

use std::{io,env};
use std::path::Path;
use std::fmt::Debug;

use clap::{App, SubCommand, Arg};

pub mod naive;
use naive::GetBit;

fn puts<F>(input:F) where F: Debug { println!("{:?}",input);}

fn main() {


    let matches = App::new("LISE")
        .version(&crate_version!()[..])
        .about("low intelligence steganographic embedder")
        .subcommand(SubCommand::with_name("read")
                    .about("reads the embedded information from a picture")
                    .arg( Arg::with_name("path")
                          .help("which file to use")
                          .required(true))
                   )

        .subcommand(SubCommand::with_name("write")
                    .about("embeddes information in a picture")
                    .arg( Arg::with_name("path")
                          .required(true)
                          .help("which file to use"))
                    .arg( Arg::with_name("message")
                          .required(true)
                          .help("enter your secret message here"))
                    .arg( Arg::with_name("output")
                          .short("o")
                          .long("output")
                          .takes_value(true)
                          .help("what file to write to"))
                   )
        .arg_required_else_help(true)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("read") {
        let path = Path::new(matches.value_of("path").unwrap());
        naive::read(Path::new(&path));
    }

    if let Some(matches) = matches.subcommand_matches("write") {
        let path = Path::new(matches.value_of("path").unwrap());
        let out_path = Path::new(matches.value_of("output").unwrap_or("./lsb.png"));
        let message = matches.value_of("message").unwrap();
        naive::write(
            Path::new(&path),
            out_path,
            &message
            )
            .ok().expect(&format!("unable to write in file {:?}", path));
    }

}
