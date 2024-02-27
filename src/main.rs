use clap::{Arg, ArgMatches, Command};
use regex::Regex;
use std::{fs::File, io::BufReader, io::BufRead};

fn main() {
   let args = parse_args(); 

   let pattern = args.get_one::<String>("pattern").unwrap();
   let re = Regex::new(&pattern).unwrap();

   let input = args.get_one::<String>("input").unwrap();
   let f = File::open(input).unwrap();
   let reader = BufReader::new(f);

   print_matches(reader, &re);
}

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
    .arg(
        Arg::new("input")
            .help("The input file to search")
            .required(true),
    )
    .get_matches();
}

fn print_matches(reader: BufReader<File>, re: &Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        if re.is_match(&line) {
            println!("{}", line);
        }
    }
}
