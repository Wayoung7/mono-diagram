use clap_derive::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Add a prefix to each line in the output
    ///
    /// This is useful when you want to paste the diagram to code comments
    #[arg(short, long, value_name = "PREFIX")]
    pub prefix: Option<String>,
    /// Copy the output to your computer clipboard
    #[arg(short, long)]
    pub copy: bool,
    /// The path to the input file
    #[arg()]
    pub file_path: String,
}
