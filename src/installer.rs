use tokio::fs;
use anyhow::{anyhow, Result};
use tokio::process::Command;

const ESPRESSIF_REPO: &str = "https://github.com/espressif/esp-idf.git";

pub struct Installer {
    version: String,
}

impl Installer {
    pub fn new(version: String) -> Self {
        Installer {
            version
        }
    }
    
    async fn get_available_branches() -> Result<Vec<String>> {
        let output = Command::new("git")
            .arg("ls-remote")
            .arg("--heads")
            .arg(ESPRESSIF_REPO)
            .output()
            .await?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        let branches = stdout.lines()
            .map(|line| line.split('/').last().unwrap().to_string())
            .collect();
        
        Ok(branches)
    }

    pub async fn install(&self) -> Result<()> {
        if !Self::get_available_branches().await?.contains(&self.version) {
            return Err(anyhow!("Version {} not found in espressif repository", self.version));
        }
        
        println!("Installing espressif {}", self.version);

        fs::create_dir_all(format!("{}/espressif/{}", env!("HOME"), &self.version)).await?;

        println!("Cloning repository");
        Command::new("sh")
            .arg("-c")
            .arg(format!("cd {}/espressif/{} && git clone {} --branch {} --recursive", env!("HOME"), &self.version, ESPRESSIF_REPO, &self.version))
            .output()
            .await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn get_branches() {
        let branches = super::Installer::get_available_branches().await.unwrap();
        println!("{:?}", branches);
        assert!(branches.contains(&"v5.0".to_string()));
    }
}