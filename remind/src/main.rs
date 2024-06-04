use anyhow::Result;
use clap::{Parser, Subcommand};
use remind::{config, scan};
use std::env;
use tokio::fs;
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
            let notes = scan::scan_notes(c.note_paths.clone(), ".md").await?;
            if notes.is_empty() {
                warn!("No notes found");
                return Ok(());
            }
            let rand = utils::random_in_range(notes.len() as u32);
            let selected = &notes[rand as usize];
            // read note content
            let md_content = fs::read_to_string(&selected.note_path).await?;
            let html_content = remind::mk2html::markdown2html(&md_content)?;
            utils::mail::send_email(
                c.email_from.as_str(),
                c.email_to.as_str(),
                c.smtp_server.as_str(),
                c.smtp_username.as_str(),
                c.smtp_password.as_str(),
                &selected.note_name,
                html_content,
            )?;
        }
        Commands::Web => {
            todo!()
        }
    }

    Ok(())
}
