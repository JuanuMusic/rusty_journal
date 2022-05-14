use anyhow::anyhow;
use structopt::StructOpt;
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // Get the command line
    let CommandLineArgs {
        action,
        journal_file,
    } = cli::CommandLineArgs::from_args();

    // Unpack journal file
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .expect("Failed to find journal file");

    // Perform the action
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;

    Ok(())
}
