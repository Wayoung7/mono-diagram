use clap_derive::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
    #[arg(short, long, value_name = "PREFIX")]
    pub prefix: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Build {
        #[clap(help = "file to build")]
        file: String,
    },
    Print {
        #[clap(help = "file to print")]
        file: String,
    },
    Watch {
        #[clap(help = "file to watch")]
        file: String,
    },
    Copy {
        #[clap(help = "file to copy")]
        file: String,
    },
}
