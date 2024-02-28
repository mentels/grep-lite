use clap::{Arg, ArgMatches, Command};
use regex::Regex;
use std::{fs::File, io, io::BufRead, io::BufReader};

fn parse_args() -> ArgMatches {
    return Command::new("grep-lie")
        .author("Me,me@mail.com")
        .about("searches for pattern in a file")
        .version("0.1")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .required(true),
        )
        .arg(Arg::new("input").help("The input file to search"))
        .get_matches();
}

fn print_matches<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = parse_args();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let re = Regex::new(&pattern).unwrap();

    let input = args.get_one::<String>("input");

    match input {
        Some(input) => {
            let f = File::open(input).unwrap();
            let reader = BufReader::new(f);
            print_matches(reader, re);
        }
        None => {
            let stdin = io::stdin();
            let reader = stdin.lock();
            print_matches(reader, re);
        }
    }
}
