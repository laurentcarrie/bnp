use std::process::ExitCode;

use clap::{Parser, Subcommand};
use lopdf::Document;

use crate::jobs::csv::tocsv;
use jobs::check::check;
use jobs::scan::scan;

use crate::jobs::scan::initjson;

// use crate::sched::cli::{blah_handle_cli, BlahArgs};

mod jobs;
mod util;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // #[command(subcommand_help_heading = "run jobs", name = "jobs")]
    // JobsGroup(JobsArgs),
    #[command(subcommand_help_heading = "scan", name = "scan")]
    Scan(StringArg), // #[command(subcommand_help_heading = "blah", name = "blah")]
    InitJson(StringArg),
    // BlahGroup(BlahArgs),
    Check(StringArg),
    Xxx(StringArg),
    ToCsv(StringArg),
}
#[derive(Parser)]
pub struct IntArg {
    arg: u32,
}
#[derive(Parser)]
pub struct StringArg {
    arg: String,
}
fn main() -> ExitCode {
    let cli = Cli::parse();

    let exit_code = match cli.cmd {
        // Commands::JobsGroup(x) => jobs_group_handle_cli(x), // Commands::BlahGroup(x) => blah_handle_cli(x),
        Commands::Scan(s) => {
            // println!("s is {}", s.arg);
            match scan(s.arg).is_ok() {
                true => 0,
                false => 1,
            }
        }
        Commands::InitJson(s) => match initjson(s.arg) {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("InitJson {}", err);
                1
            }
        },
        Commands::ToCsv(s) => match tocsv(s.arg) {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("tocsv {}", err);
                1
            }
        },
        Commands::Check(s) => {
            // println!("s is {}", s.arg);
            match check(s.arg) {
                Ok(_b) => 0,
                Err(msg) => {
                    println!("{}", msg);
                    1
                }
            }
        }
        Commands::Xxx(s) => {
            let x = s.arg;
            // let bytes = std::fs::read(x).unwrap();
            // // dbg!(&bytes) ;
            // let out = pdf_extract::extract_text_from_mem(&bytes) ;
            // dbg!(out) ;
            let document = Document::load(x.as_str());
            dbg!(&document);
            let mut document = document.unwrap();
            document.version = "1.4".to_string();
            // document.replace_text(1, "Hello World!", "Modified text!");
            document.save("modified.pdf").unwrap();
            1
        }
    };

    ExitCode::from(exit_code)
}
