use clap::{Parser, Subcommand, ValueEnum};
use std::fmt;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

mod util;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(default_value_t = ExportFormat::Txt)]
    export_format: ExportFormat,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ExportFormat {
    /// Txt - Default
    ///
    Txt,
}

impl fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportFormat::Txt => write!(f, "txt"),
            // ExportFormat::Json => write!(f, "json"),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    ExportDir {
        /// Sets the source
        #[arg(short, long, value_name = "DIR")]
        input: PathBuf,

        /// Sets the source
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    match &cli.command {
        Some(Commands::ExportDir { input, output }) => {
            debug!("export-dir: input path : {}", input.display());
            debug!("export-dir: output file : {}", output.display());

            let full_filelist = util::list_files(input);
            let mut gptignore_file = input.clone();
            gptignore_file.push(".gptignore");
            let ignorelist = util::get_ignorelist(&gptignore_file);
            let filelist = util::filter_paths(input.as_ref(), full_filelist, ignorelist);
            match &cli.export_format {
                ExportFormat::Txt => {
                    let list = util::concat_file_contents_with_separator(input, &filelist);
                    let s = list;
                    // print!("{}", s.to_string());
                    fs::write(output, s).expect("This to work");
                }
            }
        }
        None => {}
    }
}
