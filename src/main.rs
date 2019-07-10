extern crate xar;
use clap::{App, Arg, ArgMatches, SubCommand};
use failure::{Error, Fail};
use std::fs::File;
use xar::Archive;
use xmltree::*;
use std::path::PathBuf;

#[derive(Fail, Debug)]
enum Errors {
    #[fail(display = "Argument missing.")]
    ArgMissing,
    #[fail(display = "File ‘{}’ doesn't exist in archive ‘{}’.", _0, _1)]
    FileMissing(String, String),
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
            SubCommand::with_name("dump-file")
                .about("Dumps all metadata of the given file.")
                .arg(
                    Arg::with_name("ARCHIVE")
                        .help("The archive to read from.")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("FILE")
                        .help("While file to dump.")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("json")
                        .long("json")
                        .help("Export as JSON."),
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
                )
                .arg(
                    Arg::with_name("PATH")
                        .help("The path to list.")
                        .index(2),
                )
                .arg(
                    Arg::with_name("long")
                        .short("l")
                        .long("long")
                        .help("Show verbose output.")
                )
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("Recurse into directories.")
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
        ("dump-file", Some(matches)) => dump_file(matches),
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

fn dump_file(matches: &ArgMatches) -> Result<(), Error> {
    let archive_name = matches.value_of("ARCHIVE").ok_or(Errors::ArgMissing)?;
    let mut file = File::open(archive_name)?;

    let archive = Archive::from_read(&mut file)?;

    let filename = matches.value_of("FILE").ok_or(Errors::ArgMissing)?;

    let path = PathBuf::from(filename);

    let files = archive.toc().files()?;

    // TODO fix error.
    let file = files.find(&path).ok_or(Errors::FileMissing(filename.into(), archive_name.into()))?;


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

    let long = matches.is_present("long");
    let all = matches.is_present("all");

    let files = archive.toc().files()?;

    list_files(files, all, long)?;

    Ok(())
}

fn list_files(files: xar::toc::Files, recurse: bool, long: bool) -> Result<(), Error> {
    for file in files.iter() {
        let attrs = file.attrs();

        if long {
        } else {
            if let Some(name) = attrs.name {
                if recurse && file.path.components().count() != 0 {
                    println!("{}/{}", file.path.display(), name);
                } else {
                    println!("{}", name);
                }
            }
        }

        if recurse {
            list_files(file.files(), recurse, long)?;
        }
    }

    Ok(())
}

fn default(_matches: &ArgMatches) -> Result<(), Error> {
    Ok(())
}
