use tokio::fs;
use anyhow::{anyhow, Result};
use tokio::process::Command;

const ESPRESSIF_REPO: &str = "https://github.com/espressif/esp-idf.git";

pub struct Installer {
    version: String,
    tools: Vec<String>,
}

impl Installer {
    pub fn new(version: String, tools: Vec<String>) -> Self {
        Installer {
            version,
            tools,
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
    
    /// Creates a string of the tools seperated by commas
    fn tools_list(&self) -> String {
        let mut result = String::new();
        
        for tool in &self.tools {
            result.push_str(tool);
            result.push(',');
        }
        
        result.pop();
        result
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

        if self.tools.is_empty() {
            return Err(anyhow!("No tools specified for installation"));
        }
        
        println!("Installing tools: {:?}", self.tools);
        let idf_tools_path = format!("{}/espressif/{}/.espressif/", env!("HOME"), &self.version);
        
        Command::new("sh")
            .arg("-c")
            .arg(format!("export IDF_TOOLS_PATH={} && cd {}/espressif/{}/esp-idf && ./install.sh {}",
                         idf_tools_path,
                         env!("HOME"),
                         &self.version,
                         self.tools_list()))
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