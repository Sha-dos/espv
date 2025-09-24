mod installer;
mod manager;

use crate::installer::Installer;
use clap::{Parser, Subcommand};
use manager::Manager;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Install {
        version: Option<String>,

        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        tools: Option<Vec<String>>,
    },
    UnInstall {
        version: String,
    },
    Use {
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
        Commands::Use { version } => {
            let manager = Manager::new(version);

            manager.use_version().await?;
        }
        Commands::UnInstall { version } => {
            let manager = Manager::new(version);

            manager.uninstall().await?;
        }
    }

    Ok(())
}
