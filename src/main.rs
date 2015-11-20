extern crate term;
extern crate git2;

use std::env;
use std::process::exit;
use term::StdoutTerminal;
use git2::{Repository,Reference, RepositoryState};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
Minimal powerline implemented in rust

Usage:
  rustline left
  rustline right
  rustline (-h | --help)
  rustline --version

Options:
  -h --help                   Show this screen.
  --version                   Show version.
  --last_exit_code=<code>     Set last exit code [default: 0].
  --last_pipe_status=<codes>  Set last exit status for all elements in the pipe [default: 0].
  --shortened_path=<path>     Set short path (home shortened as ).
  --jobnum=<number>   Set number of background jobs running [default: 0].

";

const FG_NAME: u16 = term::color::BRIGHT_WHITE;
const BG_NAME: u16 = term::color::CYAN;
const FG_PATH: u16 = term::color::WHITE;
const BG_PATH: u16 = term::color::BRIGHT_BLACK;

struct Config {
    flag_shortened_path: String,
    flag_last_exit_code: String,
    flag_last_pipe_status: String,
    flag_jobnum: String,
    cmd_left: bool,
    cmd_right: bool,
}

fn print_usage_and_exit(exit_code: i32) {
    println!("{}", USAGE);
    exit(exit_code);
}

fn write_left(cout: &mut Box<StdoutTerminal>, conf: &Config) {

        write!(cout, "%{{");
        cout.fg(FG_NAME).unwrap();
        cout.bg(BG_NAME).unwrap();
        cout.attr(term::Attr::Bold).unwrap();
        write!(cout, "%}}");
        let username = match env::var("USER") {
            Ok(val) => val,
            Err(_) => "".to_string(),
        };

        write!(cout, " {} ", username);

        write!(cout, "%{{");
        assert!(cout.reset().unwrap());
        cout.fg(BG_NAME).unwrap();
        cout.bg(BG_PATH).unwrap();
        write!(cout, "%}}");
        write!(cout, "");
        write!(cout, "%{{");
        cout.fg(FG_PATH).unwrap();
        write!(cout, "%}}");


        match conf.flag_shortened_path.rfind("") {
            None => {
                write!(cout, "%{{");
                cout.attr(term::Attr::Bold).unwrap();
                write!(cout, "%}}");
                write!(cout, " {} ", &conf.flag_shortened_path);
            },
            Some(i) => {
                write!(cout, " {}", &conf.flag_shortened_path[0..i+3]);
                write!(cout, "%{{");
                cout.attr(term::Attr::Bold).unwrap();
                write!(cout, "%}}");
                write!(cout, "{} ", &conf.flag_shortened_path[i+3..]);
            }
        };
        /*
        if conf.flag_shortened_path.len() <= 0 {
            write!(cout, " missing! ");
        } else {
            write!(cout, " {} ", conf.flag_shortened_path);
        }
        // */

        write!(cout, "%{{");
        assert!(cout.reset().unwrap());
        write!(cout, "%}}");

        // if jobs are present
        if conf.flag_jobnum != "0" {
            write!(cout, "%{{");
            cout.fg(BG_PATH).unwrap();
            cout.bg(term::color::YELLOW).unwrap();
            write!(cout, "%}}");
            write!(cout, "");
            write!(cout, "%{{");
            cout.fg(term::color::BRIGHT_YELLOW).unwrap();
            cout.bg(term::color::YELLOW).unwrap();
            write!(cout, "%}}");
            write!(cout, " {} ", conf.flag_jobnum);
            write!(cout, "%{{");
            assert!(cout.reset().unwrap());
            cout.fg(term::color::YELLOW).unwrap();
            write!(cout, "%}}");
            write!(cout, " ");
        } else {
            write!(cout, "%{{");
            assert!(cout.reset().unwrap());
            cout.fg(BG_PATH).unwrap();
            write!(cout, "%}}");
            write!(cout, " ");
        }
}

fn write_right(cout: &mut Box<StdoutTerminal>, conf: &Config) {
    let dir = env::current_dir().unwrap();

    match Repository::discover(dir) {
        Ok(repo) => {
            match repo.head() {
                Ok(reference) => {
                    let reference = reference;
                    let reference = match reference.shorthand() {
                        Some(name) => {
                            name
                        },
                        None => {
                            "##missing##"
                        },
                    };

                    write!(cout, "%{{");
                    //assert!(cout.reset().unwrap());
                    cout.fg(term::color::BRIGHT_BLACK).unwrap();
                    write!(cout, "%}}");
                    write!(cout, "");
                    write!(cout, "%{{");
                    cout.fg(term::color::WHITE).unwrap();
                    cout.bg(term::color::BRIGHT_BLACK).unwrap();
                    write!(cout, "%}}");
                    write!(cout, "  {} ", reference);

                    let status = match repo.state() {
                        RepositoryState::Clean => "",
                        RepositoryState::Merge => "Merge ",
                        RepositoryState::Revert => "Revert ",
                        RepositoryState::CherryPick => "CherryPick ",
                        RepositoryState::Bisect => "Bisect ",
                        RepositoryState::Rebase => "Rebase ",
                        RepositoryState::RebaseInteractive => "RebaseInteractive ",
                        RepositoryState::RebaseMerge => "RebaseMerge ",
                        RepositoryState::ApplyMailbox => "ApplyMailbox ",
                        RepositoryState::ApplyMailboxOrRebase => "ApplyMailboxOrRebase ",
                    };

                    if status != "" {
                        write!(cout, "%{{");
                        cout.fg(term::color::YELLOW).unwrap();
                        write!(cout, "%}}");
                        write!(cout, "{}", status);
                    }
                },
                Err(_) => {},
            };
        },
        Err(_) => {},
    };

    if conf.flag_last_pipe_status != "0" {

        write!(cout, "%{{");
        cout.fg(term::color::RED).unwrap();
        //cout.bg(term::color::BLACK).unwrap();
        write!(cout, "%}}");
        write!(cout, "");
        write!(cout, "%{{");
        cout.fg(term::color::WHITE).unwrap();
        cout.bg(term::color::RED).unwrap();
        write!(cout, "%}}");
        write!(cout, " {} ", conf.flag_last_pipe_status);

    }
}

fn main() {
    //println!("\n############################");
    let mut cout = term::stdout().unwrap();

    let mut conf = Config {
        flag_shortened_path: "".to_string(),
        flag_last_exit_code: "0".to_string(),
        flag_last_pipe_status: "0".to_string(),
        flag_jobnum: "0".to_string(),
        cmd_left: false,
        cmd_right: false,
    };

    // Prints each argument on a separate line
    for (index, argument) in env::args().enumerate() {
        // println!("Evaluating argument: \"{}\" ", argument);
        if index == 0 {
            continue;
        }

        let argument = argument.trim();
        let argument_command: &str;
        let argument_option: &str;
        match argument.find('=') {
            None => {
                argument_command = &argument[..];
                argument_option = &"";
            }
            Some(i) => {
                argument_command = &argument[0..i];
                argument_option = &argument[i+1..];
            }
        };


        match argument_command {
            "--version" => {
                writeln!(cout, "Version {}", VERSION);
                exit(0);
            },
            "-h" => print_usage_and_exit(0),
            "--help" => print_usage_and_exit(0),
            "left" => conf.cmd_left = true,
            "right" => conf.cmd_right = true,
            "--last_exit_code" => conf.flag_last_exit_code = argument_option.to_string(),
            "--last_pipe_status" => conf.flag_last_pipe_status = argument_option.replace(" ", "  "),
            //"--shortened_path" => conf.flag_shortened_path = argument_option.to_string(),
            "--shortened_path" => {
                if argument_option == "/" {
                    // exepctional case -> only "/"
                    conf.flag_shortened_path = argument_option.to_string();
                    break;
                }

                conf.flag_shortened_path = argument_option.replace("/", "  ");
                if conf.flag_shortened_path.starts_with("  ") {
                    // it starts from the root and not from "~"
                    conf.flag_shortened_path.insert(0, '/');
                }
            },
            "--jobnum" => conf.flag_jobnum = argument_option.trim().to_string(),
            _ => {
                write!(cout, "Error unsupported option: \"{}\"\n", argument_command);
                print_usage_and_exit(1);
            }
        }
    }

    if conf.cmd_left {
        write_left(&mut cout, &conf);
    } else if conf.cmd_right {
        write_right(&mut cout, &conf);
    } else {
        writeln!(cout, "ERROR! incorrect usage: left or right missing");
        print_usage_and_exit(1);
    }

    write!(cout, "%{{");
    assert!(cout.reset().unwrap());
    write!(cout, "%}}");
    //cout.flush().unwrap();
}
