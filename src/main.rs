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
            SubCommand::with_name("dump-toc")
                .about("Dumps the table of contents on stdout.")
                .arg(
                    Arg::with_name("FILE")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("compact")
                        .short("c")
                        .long("compact")
                        .help("Don't pretty-print the TOC.")
                )
        )
        .subcommand(
            SubCommand::with_name("dump-header")
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
                    Arg::with_name("FILE")
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
        ("dump-header", Some(matches)) => dump_header(matches),
        ("dump-toc", Some(matches)) => dump_toc(matches),
        ("list", Some(matches)) => list(matches),
        (_, None) => default(&matches),
        (_, _) => unreachable!(),
    }
}

fn dump_toc(matches: &ArgMatches) -> Result<(), Error> {
    let filename = matches.value_of("FILE").ok_or(Errors::ArgMissing)?;
    let mut file = File::open(filename)?;

    let archive = Archive::from_read(&mut file)?;

    let stdout = std::io::stdout();
    let handle = stdout.lock();

    let config = EmitterConfig::new()
        .perform_indent(!matches.is_present("compact"))
        .indent_string("  ");

    archive.toc().data().write_with_config(handle, config)?;

    Ok(())
}
fn dump_header(matches: &ArgMatches) -> Result<(), Error> {
    let filename = matches.value_of("FILE").ok_or(Errors::ArgMissing)?;
    let mut file = File::open(filename)?;

    let archive = Archive::from_read(&mut file)?;

    if matches.is_present("json") {
        println!("{}", archive.header().to_json()?);
    } else {
        println!("{}", &archive);
    }

    Ok(())
}

fn list(matches: &ArgMatches) -> Result<(), Error> {
    let filename = matches.value_of("FILE").ok_or(Errors::ArgMissing)?;
    let mut file = File::open(filename)?;

    let archive = Archive::from_read(&mut file)?;

    for file in archive.toc().files()?.iter() {
        println!("name {:?}", file.name());
        println!("id {:?}", file.id());
        println!("type {:?}", file.ftype());
        println!("user {:?}", file.user());
        println!("group {:?}", file.group());
        println!("uid {:?}", file.uid());
        println!("gid {:?}", file.gid());
        println!("deviceno {:?}", file.deviceno());
        println!("inode {:?}", file.inode());
        println!("length {:?}", file.length());
        println!("offset {:?}", file.offset());
        println!("size {:?}", file.size());
    }

    Ok(())
}

fn default(_matches: &ArgMatches) -> Result<(), Error> {
    Ok(())
}
