use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct JobsArgs {
    #[command(subcommand)]
    sub: JobsGroupSubcommand,
}

#[derive(Subcommand)]
pub enum JobsGroupSubcommand {
    Run { command: String },
}

pub fn _jobs_group_handle_cli(_x: JobsArgs) -> u8 {
    0
}
