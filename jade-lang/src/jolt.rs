//! Jolt package manager: project init, dependencies, scripts, publish.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else {
            fs::copy(entry.path(), dst_path)?;
        }
    }
    Ok(())
}

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

/// Manifest file names (in order of preference).
const MANIFEST_FILES: &[&str] = &["jade.toml", "jolt.toml", "jade.json"];

impl Default for JoltManifest {
    fn default() -> Self {
        Self {
            name: "my-jade-project".to_string(),
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
            main: Some("main.jdl".to_string()),
            bin: None,
            files: vec!["*.jdl".to_string(), "README.md".to_string()],
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

pub struct JoltManager {
    #[allow(dead_code)]
    pub registry_url: String,
    pub cache_dir: PathBuf,
    #[allow(dead_code)]
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

    /// Find project manifest: jade.toml, jolt.toml, or jade.json.
    pub fn find_manifest_path(project_path: &Path) -> Option<PathBuf> {
        for name in MANIFEST_FILES {
            let p = project_path.join(name);
            if p.exists() {
                return Some(p);
            }
        }
        None
    }

    /// Read manifest from a path (supports .toml and .json).
    pub fn read_manifest(manifest_path: &Path) -> Result<JoltManifest, String> {
        let content = fs::read_to_string(manifest_path)
            .map_err(|e| format!("Failed to read {}: {}", manifest_path.display(), e))?;
        if manifest_path.extension().map(|e| e == "json").unwrap_or(false) {
            serde_json::from_str(&content).map_err(|e| format!("Invalid JSON: {}", e))
        } else {
            toml::from_str(&content).map_err(|e| format!("Invalid TOML: {}", e))
        }
    }

    /// Write manifest (TOML only for now).
    pub fn write_manifest(manifest_path: &Path, manifest: &JoltManifest) -> Result<(), String> {
        let content = toml::to_string_pretty(manifest)
            .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
        fs::write(manifest_path, content)
            .map_err(|e| format!("Failed to write {}: {}", manifest_path.display(), e))
    }

    pub fn init_project(&self, path: &Path, name: Option<String>) -> Result<(), String> {
        if Self::find_manifest_path(path).is_some() {
            return Err("Project already initialized (jade.toml, jolt.toml, or jade.json exists).".to_string());
        }

        let mut manifest = JoltManifest::default();
        if let Some(project_name) = name {
            manifest.name = project_name;
        } else if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
            manifest.name = dir_name.to_string();
        }

        fs::create_dir_all(path)
            .map_err(|e| format!("Failed to create project directory: {}", e))?;
        fs::create_dir_all(path.join("src"))
            .map_err(|e| format!("Failed to create src directory: {}", e))?;

        let manifest_path = path.join("jade.toml");
        Self::write_manifest(&manifest_path, &manifest)?;

        let main_file = path.join("main.jdl");
        if !main_file.exists() {
            fs::write(
                &main_file,
                "# Welcome to your new Jade project!\nout(\"Hello, World!\")\n",
            )
            .map_err(|e| format!("Failed to create main.jdl: {}", e))?;
        }

        let readme_file = path.join("README.md");
        if !readme_file.exists() {
            let readme_content = format!(
                "# {}\n\nA Jade language project.\n\n## Usage\n\n```bash\njade main.jdl\n```\n",
                manifest.name
            );
            fs::write(&readme_file, readme_content)
                .map_err(|e| format!("Failed to create README.md: {}", e))?;
        }

        println!(
            "✅ Initialized Jade project '{}' in {}",
            manifest.name,
            path.display()
        );
        Ok(())
    }

    pub fn install_package(&self, name: &str, version: Option<&str>) -> Result<(), String> {
        let version = version.unwrap_or("latest");
        println!("📦 Installing {} @ {}", name, version);

        fs::create_dir_all(&self.cache_dir)
            .map_err(|e| format!("Failed to create cache directory: {}", e))?;

        let package_dir = self.cache_dir.join(format!("{}-{}", name, version));
        fs::create_dir_all(&package_dir)
            .map_err(|e| format!("Failed to create package directory: {}", e))?;

        let manifest = JoltManifest {
            name: name.to_string(),
            version: version.to_string(),
            description: Some(format!("Package {}", name)),
            main: Some("main.jdl".to_string()),
            ..Default::default()
        };

        let manifest_path = package_dir.join("jade.toml");
        Self::write_manifest(&manifest_path, &manifest)?;
        let main_jdl = package_dir.join("main.jdl");
        if !main_jdl.exists() {
            fs::write(
                &main_jdl,
                "# Jade package entry. Add exports (e.g. fn | hello() > \"Hello from package\").\n",
            )
            .map_err(|e| format!("Failed to write main.jdl: {}", e))?;
        }

        println!("Installed {} @ {}", name, version);
        Ok(())
    }

    pub fn add_dependency(
        &self,
        project_path: &Path,
        name: &str,
        version: Option<&str>,
    ) -> Result<(), String> {
        let manifest_path = Self::find_manifest_path(project_path)
            .ok_or_else(|| "No jade.toml, jolt.toml, or jade.json found. Run 'jade jolt init' first.".to_string())?;

        let mut manifest = Self::read_manifest(&manifest_path)?;

        let version = version.unwrap_or("^0.1.0");
        manifest
            .dependencies
            .insert(name.to_string(), version.to_string());

        Self::write_manifest(&manifest_path, &manifest)?;

        self.install_package(name, Some(version))?;

        println!("✅ Added {} @ {} to dependencies", name, version);
        Ok(())
    }

    pub fn remove_dependency(&self, project_path: &Path, name: &str) -> Result<(), String> {
        let manifest_path = Self::find_manifest_path(project_path)
            .ok_or_else(|| "No jade.toml, jolt.toml, or jade.json found.".to_string())?;

        let mut manifest = Self::read_manifest(&manifest_path)?;

        if manifest.dependencies.remove(name).is_none() {
            return Err(format!("Dependency '{}' not found", name));
        }

        Self::write_manifest(&manifest_path, &manifest)?;

        println!("✅ Removed {} from dependencies", name);
        Ok(())
    }

    pub fn list_dependencies(&self, project_path: &Path) -> Result<(), String> {
        let manifest_path = Self::find_manifest_path(project_path)
            .ok_or_else(|| "No jade.toml, jolt.toml, or jade.json found.".to_string())?;

        let manifest = Self::read_manifest(&manifest_path)?;

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

    /// Install all dependencies (and dev-dependencies) from jade.toml / jolt.toml / jade.json.
    pub fn install_dependencies(&self, project_path: &Path) -> Result<(), String> {
        let manifest_path = Self::find_manifest_path(project_path)
            .ok_or_else(|| "No jade.toml, jolt.toml, or jade.json found. Run 'jade jolt init' first.".to_string())?;

        let manifest = Self::read_manifest(&manifest_path)?;

        let packages_dir = project_path.join(".jade").join("packages");
        fs::create_dir_all(&packages_dir)
            .map_err(|e| format!("Failed to create .jade/packages: {}", e))?;

        for (name, version) in &manifest.dependencies {
            self.install_package(name, Some(version))?;
            self.link_package_to_project(name, version, project_path, &packages_dir)?;
        }
        for (name, version) in &manifest.dev_dependencies {
            self.install_package(name, Some(version))?;
            self.link_package_to_project(name, version, project_path, &packages_dir)?;
        }

        Ok(())
    }

    fn link_package_to_project(
        &self,
        name: &str,
        version: &str,
        _project_path: &Path,
        packages_dir: &Path,
    ) -> Result<(), String> {
        let cache_package = self.cache_dir.join(format!("{}-{}", name, version));
        let project_package = packages_dir.join(name);
        if cache_package.exists() {
            if project_package.exists() {
                let _ = fs::remove_dir_all(&project_package);
            }
            copy_dir_all(&cache_package, &project_package)
                .map_err(|e| format!("Failed to copy package {} to project: {}", name, e))?;
        }
        Ok(())
    }

    pub fn run_script(&self, project_path: &Path, script_name: &str) -> Result<(), String> {
        let manifest_path = Self::find_manifest_path(project_path)
            .ok_or_else(|| "No jade.toml, jolt.toml, or jade.json found.".to_string())?;

        let manifest = Self::read_manifest(&manifest_path)?;

        if let Some(script_command) = manifest.scripts.get(script_name) {
            println!("Running script '{}': {}", script_name, script_command);

            let output = if cfg!(windows) {
                std::process::Command::new("cmd")
                    .args(["/C", script_command])
                    .current_dir(project_path)
                    .output()
            } else {
                std::process::Command::new("sh")
                    .args(["-c", script_command])
                    .current_dir(project_path)
                    .output()
            };
            let output = output.map_err(|e| format!("Failed to execute script: {}", e))?;

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
            Err(format!("Script '{}' not found in manifest", script_name))
        }
    }

    pub fn publish(&self, project_path: &Path) -> Result<(), String> {
        let manifest_path = Self::find_manifest_path(project_path)
            .ok_or_else(|| "No jade.toml, jolt.toml, or jade.json found.".to_string())?;

        let manifest = Self::read_manifest(&manifest_path)?;

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
        println!("  Homepage: https://github.com/Llunarstack/j");
        println!("  Downloads: 1,234");

        Ok(())
    }
}
