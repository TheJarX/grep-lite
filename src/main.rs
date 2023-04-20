use clap::{App, Arg};
use colored::Colorize;
use regex::Regex;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();

        match re.find(&line) {
            Some(mat) => println!(
                "{}",
                line.replace(
                    &re.as_str(),
                    &line[mat.start()..mat.end()]
                        .on_yellow()
                        .bold()
                        .red()
                        .to_string()
                )
            ),
            None => (),
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("Searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .required(true)
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::with_name("file")
                .help("The file to search in")
                .takes_value(true)
                .index(2),
        )
        .get_matches();

    let re = Regex::new(args.value_of("pattern").unwrap()).unwrap();

    let input_for_file = args.value_of("file").unwrap_or("-");
    if input_for_file == "-" {
        let stdin = stdin();
        let reader = stdin.lock();
        process_lines(reader, re)
    } else {
        let f = File::open(input_for_file).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re)
    }
}
