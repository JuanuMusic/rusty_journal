use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file
    Add {
        /// The task description.
        #[structopt()]
        text: String,
    },

    /// Mark an entry as completed by removing it from the journal file by position
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all the tasks on the journal file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line TO-DO app written in Rust as a learning excercise."
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
