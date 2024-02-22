use regex::Regex;
fn main() {
    // left mutable since the "io version requires that"
    let mut search_term: String;
    #[cfg(read_line)]
    {
        use std::io;
        use std::io::Write;
        print!("Enter a search term: ");
        search_term = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut search_term)
            .expect("Failed to read search term");
    }
    #[cfg(not(read_line))]
    {
        use clap::{Arg, Command};
        let matches = Command::new("grep")
            .arg(
                Arg::new("pattern")
                    .help("The pattern to search for")
                    .required(true),
            )
            .author("Me, me@mail.com")
            .version("1.0.2")
            .get_matches();
        search_term = matches
            .get_one::<String>("pattern")
            .expect("No pattern provided")
            .to_string();
    }

    let re = Regex::new(&search_term.trim()).unwrap();

    let quote = "\
Lorem ipsum dolor sit amet,
consectetur adipiscing elit,
other
sed do eiusmod tempor incididunt
ut labore et dolore magna aliqua.";

    println!("REGEX GREP:");
    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }

    println!("GREP WITH CONTEXT:");
    let ctx_lines = 2;
    let mut lines_to_print: Vec<usize> = Vec::new();
    let mut ctx: Vec<(usize, String)> = Vec::new();

    for (i, line) in quote.lines().enumerate() {
        let line_no = i + 1;
        ctx.push((line_no, line.to_string()));
        if line.contains(&search_term.trim()) {
            lines_to_print.push(line_no);
        }
    }

    for line_no in lines_to_print {
        let lower_line_no = line_no.saturating_sub(ctx_lines).max(1); // Wrap the calculation in max function
        let upper_line_no = (line_no + ctx_lines).min(ctx.len());
        println!("{}: {} - {}", line_no, lower_line_no, upper_line_no);
        for (line_no, line) in ctx[lower_line_no - 1..=upper_line_no - 1].iter() {
            println!("{}: {}", line_no, line);
        }
    }

    println!("NORMAL GREP:");
    for (i, line) in quote.lines().enumerate() {
        if line.contains(&search_term.trim()) {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
