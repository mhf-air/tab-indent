use clap::{App, Arg};
use std::fs;
use std::path::PathBuf;

fn main() {
    // parse args
    let input_arg = Arg::new("input")
        .long("input")
        .value_name("FILE")
        .about("Set the input file");
    #[cfg(not(debug_assertions))]
    let input_arg = input_arg.required(true);
    #[cfg(debug_assertions)]
    let input_arg = input_arg.default_value("./src/main.rs");

    let matches = App::new("tab-indent")
        .version("1.0.0")
        .author("Lisper")
        .about("transform line-leading spaces into tabs")
        .arg(input_arg)
        .arg(
            Arg::new("tab-width")
                .long("tab-width")
                .takes_value(true)
                .value_name("TAB-WIDTH")
                .default_value("4")
                .about("Set the tab width"),
        )
        .arg(
            Arg::new("inplace")
                .long("inplace")
                .about("Modify the file inplace"),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let tab_width: u8 = matches.value_of_t("tab-width").unwrap();
    let inplace = matches.is_present("inplace");

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
    let mut r = String::new();
    let list: Vec<char> = text.chars().collect();

    let mut i = 0;
    let len = list.len();
    'outer: while i < len {
        if list[i] == '\n' {
            r.push(list[i]);
            i += 1;
            if i >= len {
                break;
            }

            // skip tabs
            while list[i] == '\t' {
                i += 1;
                r.push('\t');
            }

            // spaces to tabs
            if list[i] == ' ' {
                let mut space_count = 1;
                i += 1;
                while list[i] == ' ' {
                    space_count += 1;
                    i += 1;
                }
                let tabs = space_count / tab_width;
                for _ in 0..tabs {
                    r.push('\t');
                }
                let spaces = space_count % tab_width;
                for _ in 0..spaces {
                    r.push(' ');
                }
            }
            continue;
        }

        // remove line-trailling spaces
        if list[i] == ' ' {
            let mut space_count = 1;
            let mut a = i + 1;
            while a < len {
                match list[a] {
                    ' ' => {
                        space_count += 1;
                    }
                    '\n' => {
                        i += space_count;
                        continue 'outer;
                    }
                    _ => break,
                }
                a += 1;
            }
            if a >= len {
                break;
            }
        }

        r.push(list[i]);
        i += 1;
    }

    r
}
