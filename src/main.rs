mod commands;
mod poetry;
mod pyproject;

use clap::Parser;

use commands::{Arguments, SubCommand};
use poetry::{add, fix, update};

fn main() {
    let args = Arguments::parse();

    match args.sub_command {
        SubCommand::Add { packages, pin } => add(packages, pin),
        SubCommand::Fix { dry_run, pin } => fix(dry_run, pin),
        SubCommand::Update { pin } => update(pin),
    }
}
