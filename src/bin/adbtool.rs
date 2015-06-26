extern crate actiondb;
extern crate clap;

mod parse;
mod validate;

use clap::{Arg, App, SubCommand, ArgMatches};

const VERSION: &'static str = "0.1.0";
const AUTHOR: &'static str = "Tibor Benke <tibor.benke@balabit.com>";
const APPNAME: &'static str = "adbtool";

const PATTERN_FILE: &'static str = "pattern file";
const VALIDATE: &'static str = "validate";
const PARSE: &'static str = "parse";
const INPUT_FILE: &'static str = "input file";
const OUTPUT_FILE: &'static str = "output file";

fn build_command_line_argument_parser<'a, 'b, 'c, 'd, 'e, 'f>() -> App<'a, 'b, 'c, 'd, 'e, 'f> {
    App::new(APPNAME)
          .version(VERSION)
          .author(AUTHOR)
          .about("Tool for parsing unstructured data")
          .subcommand(SubCommand::with_name(VALIDATE)
                      .about("validates pattern file")
                      .version(VERSION)
                      .author(AUTHOR)
                      .arg(Arg::with_name(PATTERN_FILE)
                          .required(true)
                          .index(1)
                          .help("The pattern file to be validated")))
          .subcommand(SubCommand::with_name(PARSE)
                      .about("parses a file based on predefined patterns")
                      .version(VERSION)
                      .author(AUTHOR)
                      .arg(Arg::with_name(PATTERN_FILE)
                          .required(true)
                          .index(1)
                          .help("The pattern file which contains predefined patterns"))
                      .arg(Arg::with_name(INPUT_FILE)
                          .required(true)
                          .index(2)
                          .help("The input file to be parsed"))
                      .arg(Arg::with_name(OUTPUT_FILE)
                          .required(true)
                          .index(3)
                          .help("The output file where the results are written")))
}

fn handle_validate(matches: &ArgMatches) {
    let pattern_file = matches.value_of(PATTERN_FILE).unwrap();
    if !validate::validate(pattern_file) {
        std::process::exit(1);
    }
}

fn handle_parse(matches: &ArgMatches) {
    let pattern_file = matches.value_of(PATTERN_FILE).unwrap();
    let input_file = matches.value_of(INPUT_FILE).unwrap();
    let output_file = matches.value_of(OUTPUT_FILE).unwrap();

    if let Err(e) = parse::parse(pattern_file, input_file, output_file) {
        println!("{}", e);
        std::process::exit(1);
    }
}

fn main() {
    let matches = build_command_line_argument_parser().get_matches();

    if let Some(matches) = matches.subcommand_matches(VALIDATE) {
        handle_validate(&matches);
    } else if let Some(matches) = matches.subcommand_matches(PARSE) {
        handle_parse(&matches);
    } else {
        println!("{}", matches.usage.as_ref().unwrap());
    }
}