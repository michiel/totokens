use clap::{Parser, Subcommand, ValueEnum};
use std::io::Write;
use std::path::PathBuf;
use std::{fmt, fs, io};
use tracing::{debug, error, info, Level};
use tracing_subscriber::FmtSubscriber;

mod util;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(short, long, default_value_t = Level::INFO)]
    loglevel: Level,
    #[arg(short, long, default_value_t = ExportFormat::Txt)]
    export_format: ExportFormat,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ExportFormat {
    /// Txt - Default
    ///
    Txt,
    /// Tokens
    TokenP50k,
}

impl fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportFormat::Txt => write!(f, "txt"),
            ExportFormat::TokenP50k => write!(f, "tokenp50k"),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    ExportDir {
        /// Sets the source
        #[arg(short, long, value_name = "DIR")]
        input: PathBuf,

        /// Sets the output file, will print to STDOUT if not set
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    let loglevel = &cli.loglevel;

    let subscriber = FmtSubscriber::builder()
        // .with_max_level(Level::TRACE)
        .with_max_level(*loglevel)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    match &cli.command {
        Some(Commands::ExportDir { input, output }) => {
            info!("export-dir: input path : {}", input.display());
            if let Some(output) = output {
                info!("export-dir: output file : {}", output.display());
            } else {
                debug!("export-dir: output to STDOUT");
            }

            let full_filelist = util::list_files(input);
            let mut gptignore_file = input.clone();
            gptignore_file.push(".gptignore");
            let ignorelist = util::get_ignorelist(&gptignore_file);
            let filelist = util::filter_paths(input.as_ref(), full_filelist, ignorelist);

            let list = util::concat_file_contents_with_separator(input, &filelist);
            let s = list;
            match &cli.export_format {
                ExportFormat::TokenP50k => {
                    if let Some(output) = output {
                        fs::write(output, &s).expect("This to work");
                        let tokens = util::tokenise_p50k(&s);
                        info!("token count: {}", tokens.len());
                        let mut bytes = vec![];
                        for item in tokens {
                            bytes.extend_from_slice(&item.to_ne_bytes());
                        }
                        fs::write(output, &bytes).expect("This to work");
                    } else {
                        error!("Not sending raw data to STDOUT");
                    }
                }
                ExportFormat::Txt => {
                    if let Some(output) = output {
                        fs::write(output, s).expect("This to work");
                    } else {
                        let mut stdout = io::stdout();
                        stdout.write_all(s.as_bytes()).expect("This to work");
                        stdout.flush().expect("This to work");
                    }
                }
            }
        }
        None => {}
    }
}
