mod installer;

use clap::{Parser, Subcommand};
use crate::installer::Installer;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Install {
        #[arg(short, long)]
        version: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    match args.command {
        Commands::Install { version } => {
            let v = match version {
                Some(v) => {
                    println!("Installing ESP-IDF version: {}", v);
                    v
                },
                None => {
                    println!("Installing latest ESP-IDF version");
                    "main".to_string()
                },
            };
            
            let installer = Installer::new(format!("v{}", v));
            
            installer.install().await?;
        }
    }
    
    Ok(())
}
