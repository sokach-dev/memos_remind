use anyhow::Result;
use clap::{Parser, Subcommand};
use remind::{config, scan};
use std::env;
use tracing::warn;

#[derive(Debug, Parser)]
#[clap(name = "remind", version = "0.1.0", author = "_Aidan_")]
#[clap(propagate_version = false)]
struct Cli {
    #[clap(short, long, default_value = "app.toml")]
    config: String,

    #[clap(subcommand)]
    subcmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(name = "remind", about = "remind your notes")]
    Remind,

    #[clap(name = "web", about = "a simple web server")]
    Web,
}

#[tokio::main]
async fn main() -> Result<()> {
    utils::log::init_tracing();
    let cli = Cli::parse();

    let c = config::get_global_config().await;

    env::set_var("REMIND_CONFIG", cli.config.clone());

    match cli.subcmd {
        Commands::Remind => {
            let notes = scan::scan_notes(c.note_paths.clone(), ".rs").await?;
            if notes.is_empty() {
                warn!("No notes found");
                return Ok(());
            }
            let rand = utils::random_in_range(notes.len() as u32);
        }
        Commands::Web => {
            todo!()
        }
    }

    Ok(())
}
