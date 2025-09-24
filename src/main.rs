mod installer;
mod manager;

use crate::installer::Installer;
use clap::{Parser, Subcommand};
use manager::Manager;

#[derive(Parser, Debug)]
#[command(
    name = "espv",
    version,
    about = "Command line tool to manage multiple esp-idf versions at once"
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Install ESP-IDF (e.g. espv install v5.5 -- tools esp32 esp32s3)")]
    Install {
        version: Option<String>,

        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        tools: Option<Vec<String>>,
    },
    UnInstall {
        version: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Install { version, tools } => {
            let v = version.unwrap_or_else(|| {
                println!("No version specified, defaulting to 'main'");
                "main".to_string()
            });

            let tools = tools.unwrap_or_else(|| {
                println!("No tools specified, run with --tools to specify tools to install");
                vec![]
            });

            let installer = Installer::new(v, tools);

            installer.install().await?;
        }
        Commands::UnInstall { version } => {
            let manager = Manager::new(version);

            manager.uninstall().await?;
        }
    }

    Ok(())
}
