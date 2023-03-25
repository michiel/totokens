use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

mod util;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ExportFormat {
    /// Txt - Default
    Txt,
    /// Json
    Json,
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
            gptignore_file.push(".gptignore".to_string());
            let ignorelist = util::get_ignorelist(&gptignore_file);
            let filelist = util::filter_paths(full_filelist, ignorelist);

            // fs::write(output, out_string).expect("This to work");
        }
        None => {}
    }
}
