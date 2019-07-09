extern crate xar;
use clap::{App, Arg, ArgMatches, SubCommand};
use failure::{Error, Fail};
use std::fs::File;
use xar::Archive;
use xmltree::*;

#[derive(Fail, Debug)]
enum Errors {
    #[fail(display = "Argument missing.")]
    ArgMissing,
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
                )
                .arg(
                    Arg::with_name("json")
                        .long("json")
                        .help("Export header as JSON."),
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
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

fn run(matches: &ArgMatches) -> Result<(), Error> {
    match matches.subcommand() {
        ("inspect", Some(matches)) => inspect(matches),
        ("list", Some(matches)) => list(matches),
        (_, None) => default(&matches),
        (_, _) => unreachable!(),
    }
}

fn inspect(matches: &ArgMatches) -> Result<(), Error> {
    let filename = matches.value_of("FILE").ok_or(Errors::ArgMissing)?;
    let mut file = File::open(filename)?;

    let archive = Archive::from_read(&mut file)?;

    if matches.is_present("json") {
        println!("{}", archive.header().to_json()?);
    } else {
        println!("{}", &archive);
    }

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    let config = EmitterConfig::new()
        .perform_indent(true)
        .indent_string("  ");
    archive.toc().data().write_with_config(handle, config)?;

    Ok(())
}

fn list(_matches: &ArgMatches) -> Result<(), Error> {
    Ok(())
}

fn default(_matches: &ArgMatches) -> Result<(), Error> {
    Ok(())
}
