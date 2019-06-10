extern crate xar;
use clap::{App, Arg, ArgMatches, SubCommand};
use error_chain::{error_chain, ChainedError};
use std::fs::File;
use xar::Archive;

error_chain! {
    foreign_links {
        //IO(std::io::Error);
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
    let filename = matches
        .value_of("FILE")
        .chain_err(|| "No file specified.")?;
    let mut file = File::open(filename).chain_err(|| "Unable to open the archive.")?;

    let archive = Archive::from_read(&mut file).chain_err(|| "Can't inspect archive.")?;

    if matches.is_present("json") {
        println!(
            "{}",
            archive.header().to_json().chain_err(|| "Can't convert to JSON.")?
        );
    } else {
        println!("{}", &archive);
    }

    Ok(())
}

fn list(_matches: &ArgMatches) -> Result<()> {
    Ok(())
}

fn default(_matches: &ArgMatches) -> Result<()> {
    Ok(())
}
