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

        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        tools: Option<Vec<String>>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
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
            
            let installer = Installer::new(format!("v{}", v), tools);
            
            installer.install().await?;
        }
    }
    
    Ok(())
}
