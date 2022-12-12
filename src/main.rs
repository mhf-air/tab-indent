use clap::{value_parser, Arg, ArgAction, Command};
use std::fs;
use std::path::PathBuf;

fn main() {
    // parse args
    let input_arg = Arg::new("input")
        .long("input")
        .value_name("FILE")
        .help("Set the input file");
    #[cfg(not(debug_assertions))]
    let input_arg = input_arg.required(true);
    #[cfg(debug_assertions)]
    let input_arg = input_arg.default_value("./src/main.rs");

    let matches = Command::new("tab-indent")
        .version("1.0.0")
        .author("Lisper")
        .about("transform line-leading spaces into tabs, and remove line-trailing spaces")
        .arg(input_arg)
        .arg(
            Arg::new("tab-width")
                .long("tab-width")
                .value_name("TAB-WIDTH")
                .default_value("4")
                .value_parser(value_parser!(u8))
                .help("Set the tab width"),
        )
        .arg(
            Arg::new("inplace")
                .long("inplace")
                .action(ArgAction::SetTrue)
                .help("Modify the file inplace"),
        )
        .get_matches();

    let input = matches.get_one::<String>("input").unwrap();
    let tab_width = *matches.get_one::<u8>("tab-width").unwrap();
    let inplace = *matches.get_one::<bool>("inplace").unwrap();

    // fmt
    let p = fs::canonicalize(PathBuf::from(input)).unwrap();
    let text = fs::read_to_string(p).unwrap();
    let text = format_run(text, tab_width);
    if inplace {
        fs::write(input, text).unwrap();
    } else {
        println!("{}", text);
    }
}

fn format_run(text: String, tab_width: u8) -> String {
    if text.len() == 0 {
        return text;
    }

    let mut r = String::new();
    let mut is_first_line = true;
    for line in text.lines() {
        if is_first_line {
            is_first_line = false;
        } else {
            r.push('\n');
        }

        if line.is_empty() {
            continue;
        }
        let mut chars = line.chars().peekable();

        let mut leading_tabs = 0;
        while let Some('\t') = chars.peek() {
            chars.next();
            leading_tabs += 1;
        }

        // space to tabs
        let mut space_count = 0;
        while let Some(' ') = chars.peek() {
            chars.next();
            space_count += 1;
        }
        if chars.peek().is_none() {
            // all tabs or spaces in this line, remove it
            continue;
        }

        leading_tabs += space_count / tab_width;
        for _ in 0..leading_tabs {
            r.push('\t');
        }
        let spaces = space_count % tab_width;
        for _ in 0..spaces {
            r.push(' ');
        }

        let mut space_count = 0;
        loop {
            match chars.peek() {
                Some(' ') => {
                    space_count += 1;
                    chars.next();
                }
                None => {
                    break;
                }
                Some(&ch) => {
                    for _ in 0..space_count {
                        r.push(' ');
                    }
                    r.push(ch);
                    space_count = 0;
                    chars.next();
                }
            }
        }
    }

    if text.len() > 1 && text.as_bytes()[text.len() - 2] == '\n' as u8 {
        r.push('\n');
    }
    r
}
