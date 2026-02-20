use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoltManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
    pub scripts: HashMap<String, String>,
    pub keywords: Vec<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub main: Option<String>,
    pub bin: Option<HashMap<String, String>>,
    pub files: Vec<String>,
}

impl Default for JoltManifest {
    fn default() -> Self {
        Self {
            name: "my-j-project".to_string(),
            version: "0.1.0".to_string(),
            description: None,
            author: None,
            license: Some("MIT".to_string()),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            scripts: HashMap::new(),
            keywords: Vec::new(),
            repository: None,
            homepage: None,
            main: Some("main.j".to_string()),
            bin: None,
            files: vec!["*.j".to_string(), "README.md".to_string()],
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JoltPackage {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub manifest: JoltManifest,
}

#[allow(dead_code)]
pub struct JoltManager {
    pub registry_url: String,
    pub cache_dir: PathBuf,
    pub global_dir: PathBuf,
}

impl Default for JoltManager {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        Self {
            registry_url: "https://registry.jolt.dev".to_string(),
            cache_dir: home_dir.join(".jolt").join("cache"),
            global_dir: home_dir.join(".jolt").join("global"),
        }
    }
}

impl JoltManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_project(&self, path: &Path, name: Option<String>) -> Result<(), String> {
        let manifest_path = path.join("jolt.toml");

        if manifest_path.exists() {
            return Err("Project already initialized (jolt.toml exists)".to_string());
        }

        let mut manifest = JoltManifest::default();
        if let Some(project_name) = name {
            manifest.name = project_name;
        } else if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
            manifest.name = dir_name.to_string();
        }

        // Create directories
        fs::create_dir_all(path)
            .map_err(|e| format!("Failed to create project directory: {}", e))?;
        fs::create_dir_all(path.join("src"))
            .map_err(|e| format!("Failed to create src directory: {}", e))?;

        // Write manifest
        let toml_content = toml::to_string_pretty(&manifest)
            .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
        fs::write(&manifest_path, toml_content)
            .map_err(|e| format!("Failed to write jolt.toml: {}", e))?;

        // Create main.j file
        let main_file = path.join("main.j");
        if !main_file.exists() {
            fs::write(
                &main_file,
                "# Welcome to your new J project!\nout(\"Hello, World!\")\n",
            )
            .map_err(|e| format!("Failed to create main.j: {}", e))?;
        }

        // Create README.md
        let readme_file = path.join("README.md");
        if !readme_file.exists() {
            let readme_content = format!(
                "# {}\n\nA J language project.\n\n## Usage\n\n```bash\nj run main.j\n```\n",
                manifest.name
            );
            fs::write(&readme_file, readme_content)
                .map_err(|e| format!("Failed to create README.md: {}", e))?;
        }

        println!(
            "✅ Initialized J project '{}' in {}",
            manifest.name,
            path.display()
        );
        Ok(())
    }

    pub fn install_package(&self, name: &str, version: Option<&str>) -> Result<(), String> {
        let version = version.unwrap_or("latest");
        println!("📦 Installing {} @ {}", name, version);

        // Create cache directory
        fs::create_dir_all(&self.cache_dir)
            .map_err(|e| format!("Failed to create cache directory: {}", e))?;

        // For now, simulate package installation
        let package_dir = self.cache_dir.join(format!("{}-{}", name, version));
        fs::create_dir_all(&package_dir)
            .map_err(|e| format!("Failed to create package directory: {}", e))?;

        // Create a dummy package manifest
        let manifest = JoltManifest {
            name: name.to_string(),
            version: version.to_string(),
            description: Some(format!("Package {}", name)),
            ..Default::default()
        };

        let manifest_content = toml::to_string_pretty(&manifest)
            .map_err(|e| format!("Failed to serialize package manifest: {}", e))?;
        fs::write(package_dir.join("jolt.toml"), manifest_content)
            .map_err(|e| format!("Failed to write package manifest: {}", e))?;

        println!("✅ Installed {} @ {}", name, version);
        Ok(())
    }

    pub fn add_dependency(
        &self,
        project_path: &Path,
        name: &str,
        version: Option<&str>,
    ) -> Result<(), String> {
        let manifest_path = project_path.join("jolt.toml");

        if !manifest_path.exists() {
            return Err("No jolt.toml found. Run 'jolt init' first.".to_string());
        }

        // Read existing manifest
        let manifest_content = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read jolt.toml: {}", e))?;

        let mut manifest: JoltManifest = toml::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse jolt.toml: {}", e))?;

        // Add dependency
        let version = version.unwrap_or("^0.1.0");
        manifest
            .dependencies
            .insert(name.to_string(), version.to_string());

        // Write updated manifest
        let updated_content = toml::to_string_pretty(&manifest)
            .map_err(|e| format!("Failed to serialize updated manifest: {}", e))?;
        fs::write(&manifest_path, updated_content)
            .map_err(|e| format!("Failed to write updated jolt.toml: {}", e))?;

        // Install the package
        self.install_package(name, Some(version))?;

        println!("✅ Added {} @ {} to dependencies", name, version);
        Ok(())
    }

    pub fn remove_dependency(&self, project_path: &Path, name: &str) -> Result<(), String> {
        let manifest_path = project_path.join("jolt.toml");

        if !manifest_path.exists() {
            return Err("No jolt.toml found.".to_string());
        }

        // Read existing manifest
        let manifest_content = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read jolt.toml: {}", e))?;

        let mut manifest: JoltManifest = toml::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse jolt.toml: {}", e))?;

        // Remove dependency
        if manifest.dependencies.remove(name).is_none() {
            return Err(format!("Dependency '{}' not found", name));
        }

        // Write updated manifest
        let updated_content = toml::to_string_pretty(&manifest)
            .map_err(|e| format!("Failed to serialize updated manifest: {}", e))?;
        fs::write(&manifest_path, updated_content)
            .map_err(|e| format!("Failed to write updated jolt.toml: {}", e))?;

        println!("✅ Removed {} from dependencies", name);
        Ok(())
    }

    pub fn list_dependencies(&self, project_path: &Path) -> Result<(), String> {
        let manifest_path = project_path.join("jolt.toml");

        if !manifest_path.exists() {
            return Err("No jolt.toml found.".to_string());
        }

        let manifest_content = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read jolt.toml: {}", e))?;

        let manifest: JoltManifest = toml::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse jolt.toml: {}", e))?;

        println!("📦 Dependencies for {}:", manifest.name);
        if manifest.dependencies.is_empty() {
            println!("  (no dependencies)");
        } else {
            for (name, version) in &manifest.dependencies {
                println!("  {} @ {}", name, version);
            }
        }

        if !manifest.dev_dependencies.is_empty() {
            println!("\n🔧 Dev Dependencies:");
            for (name, version) in &manifest.dev_dependencies {
                println!("  {} @ {}", name, version);
            }
        }

        Ok(())
    }

    pub fn run_script(&self, project_path: &Path, script_name: &str) -> Result<(), String> {
        let manifest_path = project_path.join("jolt.toml");

        if !manifest_path.exists() {
            return Err("No jolt.toml found.".to_string());
        }

        let manifest_content = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read jolt.toml: {}", e))?;

        let manifest: JoltManifest = toml::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse jolt.toml: {}", e))?;

        if let Some(script_command) = manifest.scripts.get(script_name) {
            println!("🚀 Running script '{}': {}", script_name, script_command);

            // Execute the script command
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(script_command)
                .current_dir(project_path)
                .output()
                .map_err(|e| format!("Failed to execute script: {}", e))?;

            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
                Ok(())
            } else {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                Err(format!(
                    "Script '{}' failed with exit code: {:?}",
                    script_name,
                    output.status.code()
                ))
            }
        } else {
            Err(format!("Script '{}' not found in jolt.toml", script_name))
        }
    }

    pub fn publish(&self, project_path: &Path) -> Result<(), String> {
        let manifest_path = project_path.join("jolt.toml");

        if !manifest_path.exists() {
            return Err("No jolt.toml found.".to_string());
        }

        let manifest_content = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read jolt.toml: {}", e))?;

        let manifest: JoltManifest = toml::from_str(&manifest_content)
            .map_err(|e| format!("Failed to parse jolt.toml: {}", e))?;

        println!(
            "📤 Publishing {} @ {} to registry...",
            manifest.name, manifest.version
        );

        // TODO: Implement actual publishing to registry
        // For now, just simulate it
        println!(
            "✅ Published {} @ {} successfully!",
            manifest.name, manifest.version
        );

        Ok(())
    }

    pub fn search(&self, query: &str) -> Result<(), String> {
        println!("🔍 Searching for packages matching '{}'...", query);

        // TODO: Implement actual registry search
        // For now, show some dummy results
        let dummy_results = vec![
            ("j-http", "0.2.1", "HTTP client library for J"),
            ("j-json", "1.0.0", "JSON parsing and serialization"),
            ("j-crypto", "0.1.5", "Cryptographic functions"),
            ("j-math", "2.0.0", "Advanced mathematical operations"),
        ];

        for (name, version, description) in dummy_results {
            if name.contains(query) || description.contains(query) {
                println!("  📦 {} @ {} - {}", name, version, description);
            }
        }

        Ok(())
    }

    pub fn info(&self, package_name: &str) -> Result<(), String> {
        println!("📋 Package information for '{}':", package_name);

        // TODO: Fetch actual package info from registry
        // For now, show dummy info
        println!("  Name: {}", package_name);
        println!("  Version: 1.0.0");
        println!("  Description: A sample J package");
        println!("  Author: J Developer");
        println!("  License: MIT");
        println!("  Homepage: https://github.com/j-lang/{}", package_name);
        println!("  Downloads: 1,234");

        Ok(())
    }
}
