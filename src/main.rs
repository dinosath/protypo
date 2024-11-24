mod generator;
mod filters;
mod files;

use crate::generator::{install_template, Generator};
use anyhow::{anyhow, Error};
use clap::Parser;
use clap_derive::Subcommand;
use jsonptr::Pointer;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;

/// A fictional versioning CLI
#[derive(Parser, Debug)]
#[command(version, about="CLI application for downloading and running tera and rrgen templates", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Serialize, Deserialize, Debug)]
struct Template {
    name: String,
    version: String,
    description: String,
    dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
    name: String,
    version: String,
    repository: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Context {
    values: Value,
    entities: Value,
}

impl Default for Context {
    fn default() -> Context {
        Context{
            values: json!({}),
            entities: json!({}),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Generate {
    output: String,
}

impl Default for Generate {
    fn default() -> Generate {
        Generate{
            output: ".".to_string(),
        }
    }
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// install template to local repo
    Install {
        /// uri of the template to install
        url: String
    },
    /// create a new template scaffold
    New {
        /// the name of the new template
        name: String,
    },
    /// use generator to create output
    Generate {
        /// path to generator
        #[arg(short='p',long)]
        generator_path: Option<PathBuf>,
        /// the name of the generator
        #[arg(short, long, conflicts_with = "uri")]
        name: Option<String>,
        /// the name of the generator
        #[arg(short, long, conflicts_with = "uri")]
        version: Option<String>,
        /// uri to download and use generator
        #[arg(short='u', long, conflicts_with = "name", conflicts_with = "version")]
        uri: Option<String>,
        #[arg(long = "set")]
        sets: Vec<String>,
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let local_repo = dirs::data_local_dir().unwrap().join("protypo");
    info!("directory for protypo config and data: {:?}!", local_repo);
    let local_repo_generators = local_repo.join("generators");
    info!("directory for installing templates: {:?}!", local_repo_generators);
    match &cli.command {
        Commands::Install { url } => {
            info!("dir to install templates: {:?}!", local_repo_generators);
            install_template(&url, &local_repo_generators);
            Ok(())
        },
        Commands::New { name } => {
            info!("Creating new template: {name}");
            create_new_template(name);
            Ok(())
        },
        Commands::Generate { name,version,uri, generator_path, sets} => {
            let mut ctx = Context::default();

            let mut values = json!({});
            for set in sets {
                let parts: Vec<&str> = set.splitn(2, '=').collect();
                if parts.len() != 2 {
                    return Err(anyhow!("Invalid format for --set. Expected key=value."));
                }

                let key = parts[0];
                let val = parts[1];

                let ptr_path= key.replace(".","/");
                let ptr = Pointer::parse(ptr_path.as_str()).unwrap();
                let replaced = ptr.assign(&mut values, json!(val)).unwrap().unwrap();
                values = replaced;
            }

            let path = match true {
                true if name.is_some() && version.is_some() => {
                    let generator_name = name.clone().unwrap();
                    let generator_version = version.clone().unwrap();
                    local_repo.join("generators").join(generator_name).join(generator_version)
                },
                true if generator_path.is_some() => {
                    let path = generator_path.clone().unwrap();
                    debug!("Searching for generator in path: {} ", path.display());
                    path
                }
                true if uri.is_some() => {
                    let uri = uri.clone().unwrap();
                    debug!("Installing template from URI: {}", uri);
                    install_template(&uri, &local_repo_generators).await;
                    PathBuf::new()
                }
                _ => {
                    let error_message = "Error: Either generator name and version or a URI must be provided.";
                    error!(error_message);
                    return Err(anyhow!(error_message));
                }
            };
            let generator = Generator::from_directory(path.as_path()).await?;
            generator.copy_files(values)?;
            debug!("copied files");
            let entities = generator.collect_entities();
            ctx.entities = entities;
            generator.generate(&ctx)?;

            Ok(())
        },

    }
}

fn path_to_json(path: &PathBuf) -> Result<Value, Error> {
    fs::read_to_string(path)
        .map_err(|e| anyhow!("invalid config file path: {}", e)) // Handle file reading errors
        .and_then(|content| {
            serde_json::from_str(&content)
                .map_err(|e| anyhow!("config file is an invalid JSON: {}", e)) // Handle JSON parsing errors
        })
}

/// Function to create the new template package
fn create_new_template(name: &str) {
    // Define the directory structure and file contents
    let package_dir = format!("./{}", name);

    if Path::new(&package_dir).exists() {
        error!("Directory '{}' already exists!", name);
        return;
    }

    // Metadata file content
    let metadata = Template {
        name: name.to_string(),
        version: "0.0.1".to_string(),
        description: "A template for".to_string(),
        dependencies: vec![],
    };

    let metadata_yaml = serde_yaml::to_string(&metadata).unwrap();
    serde_yaml::to_writer(File::create(&format!("{}/template.yaml", package_dir)).unwrap(), &metadata_yaml).unwrap();
    fs::create_dir_all(&package_dir).unwrap();

    // Create the files
    create_file(&format!("{}/template.yaml", package_dir), &metadata_yaml);
    create_file(
        &format!("{}/README.md", package_dir),
        "# Template README\n\nThis is your template's README file.",
    );
    create_file(
        &format!("{}/template.html", package_dir),
        "<!-- Your template content here -->",
    );

    println!("Template package '{}' has been created!", name);
}

/// Helper function to create a file with content
fn create_file(path: &str, _content: &str) {
    // let file = File::create(path).unwrap();
    // file.write_all(content.as_bytes()).unwrap();
    println!("Created file: {}", path);
}