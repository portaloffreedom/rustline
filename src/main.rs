extern crate termion;
extern crate git2;

use std::env;
use std::process::exit;
use std::io::{Write, stdout};
use termion::{color, style};
use termion::raw::{IntoRawMode, RawTerminal};
use git2::{Repository, RepositoryState};

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

const FG_NAME: color::LightWhite = color::LightWhite;
const BG_NAME: color::Cyan = color::Cyan;
const FG_PATH: color::White = color::White;
const BG_PATH: color::LightBlack = color::LightBlack;

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

fn write_left(cout: &mut RawTerminal<std::io::Stdout>, conf: &Config) -> Result<(), std::io::Error> {
    if env::var("USER") != env::var("DEFAULT_USER") {
        write!(cout, "%{{{}{}{}%}} %n %{{{}{}{}%}}%{{{}%}} ",
            color::Fg(FG_NAME),
            color::Bg(BG_NAME),
            style::Bold,
            style::Reset,
            color::Fg(BG_NAME),
            color::Bg(BG_PATH),
            color::Fg(FG_PATH),
        )?;
    } else {
        write!(cout, "%{{{}{}{}%}} ",color::Fg(FG_PATH),color::Bg(BG_PATH),color::Fg(FG_PATH) )?;
    }

    match conf.flag_shortened_path.rfind("") {
        None => {
            write!(cout, "%{{{}{}%}}", style::Bold, color::Fg(color::LightWhite))?;
            write!(cout, "{} ", &conf.flag_shortened_path)?;
        },
        Some(i) => {
            write!(cout, "{}", &conf.flag_shortened_path[0..i + 3])?;
            write!(cout, "%{{{}{}%}}", style::Bold, color::Fg(color::LightWhite))?;
            write!(cout, "{} ", &conf.flag_shortened_path[i + 3..])?;
        }
    };
    /*
    if conf.flag_shortened_path.len() <= 0 {
        try!(write!(cout, " missing! "));
    } else {
        try!(write!(cout, " {} ", conf.flag_shortened_path));
    }
    // */

    // if jobs are present
    if conf.flag_jobnum != "0" {
        write!(cout, "%{{{}{}%}}%{{{}%}} {} %{{{}{}%}}%{{{}%}} ",
               color::Bg(color::Yellow),
               color::Fg(BG_PATH),
               color::Fg(color::LightYellow),
               conf.flag_jobnum,
               style::Reset,
               color::Fg(color::Yellow),
               style::Reset,
        )?;
    } else {
        write!(cout, "%{{{}{}%}} ",
               style::Reset,
               color::Fg(BG_PATH),
        )?;
    }

    Ok(())
}

fn write_right(cout: &mut RawTerminal<std::io::Stdout>, conf: &Config) -> Result<(), std::io::Error> {
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


                    write!(cout, "%{{{}{}%}}%{{{}{}%}}  {}",
                           style::Reset,
                           color::Fg(color::LightBlack),
                           color::Fg(color::White),
                           color::Bg(color::LightBlack),
                           reference,
                    )?;

                    if conf.flag_last_pipe_status == "0" {
                        write!(cout, " ")?;
                    }

                    let status = match repo.state() {
                        RepositoryState::Clean => None,
                        RepositoryState::Merge => Some("Merge "),
                        RepositoryState::Revert => Some("Revert "),
                        RepositoryState::CherryPick => Some("CherryPick "),
                        RepositoryState::Bisect => Some("Bisect "),
                        RepositoryState::Rebase => Some("Rebase "),
                        RepositoryState::RebaseInteractive => Some("RebaseInteractive "),
                        RepositoryState::RebaseMerge => Some("RebaseMerge "),
                        RepositoryState::ApplyMailbox => Some("ApplyMailbox "),
                        RepositoryState::ApplyMailboxOrRebase => Some("ApplyMailboxOrRebase "),
                        RepositoryState::RevertSequence => Some("RevertSequence "),
                        RepositoryState::CherryPickSequence => Some("CherryPickSequence "),
                    };

                    if let Some(status_message) = status {
                        write!(cout, "%{{{}%}}{}",
                               color::Fg(color::Yellow),
                               status_message,
                        )?;
                    }
                },
                Err(_) => {},
            };
        },
        Err(_) => {},
    };

    if conf.flag_last_pipe_status != "0" {

        write!(cout, "%{{{}%}} %{{{}{}%}} {} ",
               color::Fg(color::Red),
               color::Fg(color::White),
               color::Bg(color::Red),
               conf.flag_last_pipe_status,
        )?;

    }

    Ok(())
}

fn main() {
    let mut cout = stdout().into_raw_mode().unwrap();

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
                writeln!(cout, "Version {}", VERSION).unwrap();
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

                let separator = "  ";
                conf.flag_shortened_path = argument_option.replace("/", &separator);
                if conf.flag_shortened_path.starts_with(&separator) {
                    // it starts from the root and not from "~"
                    conf.flag_shortened_path.insert(0, '/');
                }
            },
            "--jobnum" => conf.flag_jobnum = argument_option.trim().to_string(),
            _ => {
                write!(cout, "Error unsupported option: \"{}\"\n", argument_command).unwrap();
                print_usage_and_exit(1);
            }
        }
    }

    if conf.cmd_left {
        write_left(&mut cout, &conf).unwrap();
    } else if conf.cmd_right {
        write_right(&mut cout, &conf).unwrap();
    } else {
        writeln!(cout, "ERROR! incorrect usage: left or right missing").unwrap();
        print_usage_and_exit(1);
    }

    write!(cout, "%{{{}%}}", style::Reset).unwrap();
}
