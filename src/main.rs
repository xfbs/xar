extern crate xar;
use clap::{App, Arg, SubCommand, ArgMatches};
use xar::{Header, ReadHeader};
use error_chain::{error_chain, ChainedError};
use std::fs::File;
use std::io::prelude::*;

error_chain! {
    foreign_links {
        IO(std::io::Error);
    }
}

fn main() {
    let matches = App::new("xar")
        .version("0.1.0")
        .author("Patrick Elsen <pelsen@xfbs.net>")
        .about("Create, inspect and extract XAR archives.")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("inspect")
                .about("prints the header of a XAR archive.")
                .arg(
                    Arg::with_name("FILE")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists all the files in a XAR archive.")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    match run(&matches) {
        Ok(_) => {},
        Err(e) => println!("{}", e.display_chain()),
    }
}

fn run(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("inspect", Some(matches)) => inspect(matches),
        ("list", Some(matches)) => list(matches),
        (_, None) => default(&matches),
        (_, _) => unreachable!(),
    }
}

fn inspect(matches: &ArgMatches) -> Result<()> {
    let filename = matches.value_of("FILE").chain_err(|| "No file specified.")?;
    let mut file = File::open(filename).chain_err(|| "Unable to open the archive.")?;

    let header = file.read_header();
    println!("{:?}", &header);

    Ok(())
}

fn list(matches: &ArgMatches) -> Result<()> {
    Ok(())
}

fn default(matches: &ArgMatches) -> Result<()> {
    Ok(())
}
