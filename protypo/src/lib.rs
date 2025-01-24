use glob::glob;
use json_value_merge::Merge;
use rrgen::RRgen;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::io::{Cursor, ErrorKind};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use tempfile::tempdir;
use tracing::debug;
use tracing::error;
use url::Url;
use zip::ZipArchive;

#[derive(Debug, Serialize, Deserialize)]
pub struct Generator {
    pub base_path: String,
    pub generator_yaml: GeneratorYaml,
    pub license: Option<String>,
    pub readme: Option<String>,
    pub values: Value,
    pub schema: Option<Value>,
    pub files: Option<Vec<String>>,
    pub entities: Value,
    pub templates: HashMap<String, String>,
    pub dependencies: Vec<Generator>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneratorYaml {
    #[serde(rename = "apiVersion")]
    pub api_version: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "keywords")]
    pub keywords: Option<Vec<String>>,

    #[serde(rename = "home")]
    pub home: Option<String>,

    #[serde(rename = "sources")]
    pub sources: Option<Vec<String>>,

    #[serde(rename = "dependencies")]
    pub dependencies: Option<Vec<Dependency>>,

    #[serde(rename = "maintainers")]
    pub maintainers: Option<Vec<Maintainer>>,

    #[serde(rename = "icon")]
    pub icon: Option<String>,

    #[serde(rename = "deprecated")]
    pub deprecated: Option<bool>,

    #[serde(rename = "annotations")]
    pub annotations: Option<Annotations>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotations {
    #[serde(rename = "example")]
    pub example: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "url")]
    pub repository: Url,

    #[serde(rename = "condition")]
    pub condition: Option<String>,

    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,

    #[serde(rename = "import-values")]
    pub import_values: Option<Vec<String>>,

    #[serde(rename = "alias")]
    pub alias: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Maintainer {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "email")]
    pub email: Option<String>,

    #[serde(rename = "url")]
    pub url: Option<String>,
}

impl Generator {
    fn key(&self) -> String {
        format!(
            "{}:{}",
            self.generator_yaml.name, self.generator_yaml.version
        )
    }

    /// Creates a new generator from a url.
    ///
    /// It will download the file and export it to a temporary directory if the url is from http/https. Then it will call `from_directory` in order to create a
    /// generator struct from given directory. The directory must follow
    /// the structure of a generator directory.
    ///
    /// # Errors
    ///
    /// Will return `io::Error` if `url` is wrong or the user does not have
    /// permission to read it.
    /// # Panics
    ///
    /// Will return `io::Error` if `filename` does not exist or the user does not have
    /// permission to read it.
    pub async fn from_url(url: &Url, base_path: &Path) -> Result<Self, io::Error> {
        let url_path = if url.scheme() == "file" {
            let url = url.to_string();
            debug!("url: {}", url);
            let file_path = url.strip_prefix("file://").unwrap().to_string();
            debug!("Using url is filesystem path: {}", file_path);
            let path = Path::new(&file_path);
            if path.is_relative() {
                base_path.join(path)
            } else {
                path.to_path_buf()
            }
        } else if url.scheme() == "http" || url.scheme() == "https" {
            download_and_extract_to_temp(url.clone()).await?
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported URL scheme",
            ));
        };
        Self::from_directory(&url_path).await
    }

    /// Creates a new generator from a directory.
    ///
    /// Will try to create a generator struct from given directory. The directory must follow
    /// the structure of a generator directory.
    ///
    /// # Errors
    ///
    /// Will return `io::Error` if `filename` does not exist or the user does not have
    /// permission to read it.
    /// # Panics
    ///
    /// Will return `io::Error` if `filename` does not exist or the user does not have
    /// permission to read it.
    pub async fn from_directory(base_path: &Path) -> Result<Self, io::Error> {
        debug!("Creating generator from directory: {}", base_path.display());
        let generator_yaml: GeneratorYaml = read_yaml_file(base_path, "Generator.yaml")?;
        let license = read_optional_file_as_string(base_path, "LICENSE");
        let readme = read_optional_file_as_string(base_path, "README.md");
        let values = read_yaml_file(base_path, "values.yaml")?;
        let schema = read_optional_json_file(base_path, "values.schema.json");
        let files = read_optional_directory(base_path, "files");
        let templates = glob_to_map(base_path.join("templates").join("**/*").to_str().unwrap());
        let entities = read_optional_entities(base_path, "entities")?;

        let dependencies: Vec<Self> = match &generator_yaml.dependencies {
            None => {
                vec![]
            }
            Some(dependencies) => {
                futures::future::join_all(dependencies.iter().map(|dependency| async {
                    Self::from_url(&dependency.repository, base_path)
                        .await
                        .unwrap()
                }))
                .await
            }
        };

        Ok(Self {
            base_path: base_path.to_str().unwrap().to_owned(),
            generator_yaml,
            license,
            readme,
            values,
            schema,
            files,
            entities,
            templates,
            dependencies,
        })
    }

    /// Creates a new generator from a directory.
    ///
    /// Will try to create a generator struct from given directory. The directory must follow
    /// the structure of a generator directory.
    ///
    /// # Errors
    ///
    /// Will return `io::Error` if `filename` does not exist or the user does not have
    /// permission to read it.
    /// # Panics
    ///
    /// Will return `io::Error` if `filename` does not exist or the user does not have
    /// permission to read it.
    fn get_values(&self, values: &Value) -> Value {
        let mut generator_values = self.values.clone();
        generator_values.merge(&values.clone());

        if let Some(generator_obj) = values.get(self.generator_yaml.name.clone()) {
            generator_values.merge(generator_obj);
        }
        generator_values.clone()
    }

    /// It will run the whole generation pipeline first copying the files and then generating the templates.
    ///
    /// # Errors
    ///
    /// Will return `io::Error` if an error occurs during the pipeline
    ///
    /// # Panics
    ///
    /// Will return `io::Error` if an error occurs during the pipeline
    pub fn generate(&self, ctx: &Value) -> Result<(), io::Error> {
        self.copy_files(&ctx.clone())?;

        let mut context = ctx.clone().as_object().unwrap().clone();
        context.insert("entities".to_string(), self.collect_entities());

        let binding = collect_templates(self);
        let templates: Vec<_> = binding
            .iter()
            .map(|(s1, s2)| (s1.as_str(), s2.as_str()))
            .collect();
        let rrgen = RRgen::with_templates(templates).map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Could not initialize rrgen, due to error: {err:?}"),
            )
        })?;

        self.generate_templates(&rrgen, &Value::Object(context))
    }

    fn generate_templates(&self, rrgen: &RRgen, ctx: &Value) -> Result<(), io::Error> {
        debug!(
            "Generator name:{:?},version:{:?}, base_path {:?}",
            self.generator_yaml.name, self.generator_yaml.version, self.base_path
        );
        debug!(
            "Generator name:{:?},version:{:?}, Start generating templates {:?}",
            self.generator_yaml.name, self.generator_yaml.version, self.templates
        );

        let generator_values = self.get_values(&ctx.get("values").unwrap().clone());

        let generator_context = json!({
            "values": generator_values,
            "entities": ctx.get("entities").unwrap().clone(),
        });

        let ctx = &serde_json::to_value(generator_context.clone())?;

        for dependency in &self.dependencies {
            debug!(
                "Generating templates for dependency: {:?}",
                dependency.generator_yaml.name
            );
            dependency.generate_templates(rrgen, &generator_context)?;
        }

        debug!(
            "Generator name:{:?}, version:{:?}", self.generator_yaml.name, self.generator_yaml.version
        );
        if self.templates.is_empty() {
            debug!("There are no templates to generate");
        } else {
            let templates_iter = self
                .templates
                .iter()
                .filter(|(filename, _template)| !path_is_partial(filename));
            for (filename, content) in templates_iter {
                debug!("Generator name:{:?}, version:{:?} generating template with name: {:?}, content:{:?}, context:{:?}", self.generator_yaml.name, self.generator_yaml.version, filename, content, serde_json::to_string_pretty(ctx));
                rrgen
                    .generate_by_template_with_name(filename, ctx)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            }
        }
        Ok(())
    }

    /// It will copy the files and then generating the templates.
    ///
    /// # Errors
    ///
    /// Will return `io::Error` if an error occurs during the pipeline
    ///
    /// # Panics
    ///
    /// Will return `io::Error` if an error occurs during the pipeline
    fn copy_files(&self, values: &Value) -> Result<(), io::Error> {
        let generator_values = self.get_values(values);
        for dependency in &self.dependencies {
            dependency.copy_files(&generator_values.clone())?;
        }

        let destination_dir_str = generator_values
            .as_object()
            .and_then(|obj| obj.get("outputFolder"))
            .and_then(|val| val.as_str())
            .unwrap_or(".");

        let destination_dir: &PathBuf = &Path::new(destination_dir_str).to_path_buf();

        if !destination_dir.exists() {
            fs::create_dir_all(destination_dir)?;
            debug!(
                "{} - Creating directory {:?}",
                self.key(),
                destination_dir.parent().unwrap()
            );
        }
        if !destination_dir.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Destination directory {destination_dir:?} is not a directory",),
            ));
        }

        if self.files.is_none() {
            debug!("{} - There are no files to copy", self.key());
        } else {
            debug!(
                "{} - Copying files to destination {:?}",
                self.key(),
                destination_dir
            );
            let base_path = Path::new(&self.base_path).join("files");
            for file in self.files.clone().unwrap() {
                let file_path = Path::new(&file);
                let destination =
                    construct_destination_path(&base_path, file_path, destination_dir)?;
                let destination_parent = destination.parent().unwrap();

                debug!("{} - Source file: {:?}", self.key(), file_path);
                debug!("{} - Destination file: {:?}", self.key(), destination);
                debug!(
                    "{} - Destination parent directory: {:?}",
                    self.key(),
                    destination_parent
                );

                if !destination_parent.exists() {
                    debug!(
                        "{} - Destination parent {:?} does not exist. It will be created.",
                        self.key(),
                        destination_parent
                    );
                    match fs::create_dir_all(destination_parent) {
                        Ok(()) => debug!(
                            "{} - Successfully created destination parent directory: {:?}",
                            self.key(),
                            destination_parent
                        ),
                        Err(e) => {
                            error!("{} - Failed to create destination parent directory: {:?} with error: {:?}", self.key(), destination_parent, e);
                            return Err(e);
                        }
                    }
                }

                match fs::copy(file_path, &destination) {
                    Ok(bytes_copied) => {
                        debug!(
                            "{} - Successfully copied file {:?} to {:?}. Bytes copied: {}",
                            self.key(),
                            file_path,
                            destination,
                            bytes_copied
                        );
                    }
                    Err(e) => {
                        error!(
                            "{} - Failed to copy file {:?} to {:?}: {:?}",
                            self.key(),
                            file_path,
                            destination,
                            e
                        );
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    fn collect_entities(&self) -> Value {
        let mut values = self.entities.clone();

        for dep in &self.dependencies {
            let entities = dep.collect_entities();
            values.merge(&entities);
        }
        debug!("collect entities generator:{:?} , entities: {:?}",self.generator_yaml.name, values);
        values.clone()
    }
}

/// Downloads and extracts an archive (ZIP or TAR.GZ) from a URL.
async fn download_and_extract_to_temp(url: Url) -> Result<PathBuf, io::Error> {
    let temp_dir = tempdir().unwrap().into_path();
    debug!("Downloading to {:?} to {:?}", url, temp_dir);

    let response = reqwest::get(url.clone()).await.map_err(|e| {
        io::Error::new(
            ErrorKind::Other,
            format!("Failed to download file {url:?} due to error: {e:?}"),
        )
    })?;

    if !response.status().is_success() {
        return Err(io::Error::new(
            ErrorKind::Other,
            format!(
                "Failed to download file from {}. Status code: {}",
                url,
                response.status()
            ),
        ));
    }

    let bytes = response.bytes().await.map_err(|e| {
        io::Error::new(
            ErrorKind::Other,
            format!(
                "Failed to read response bytes of file downloaded from url {url:?} due to error: {e:?}"
            ),
        )
    })?;

    let cursor = Cursor::new(bytes);
    if std::path::Path::new(&url.to_string())
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("zip"))
        || url.to_string().ends_with(".tar.gz")
    {
        let mut zip = ZipArchive::new(cursor)?;
        zip.extract(&temp_dir)?;
    }

    Ok(temp_dir)
}

fn path_is_partial(filename: &str) -> bool {
    filename.starts_with('_')
}

/// Collects the templates of the generator and the generator's dependencies with the dependencies' first so that any ancestor
/// can overwrite any descendant's template with same name
///
/// # Arguments
///
/// * `generator`:
///
/// returns: `HashMap`<String, String>
///
fn collect_templates(generator: &Generator) -> Vec<(String, String)> {
    let mut templates: Vec<(String, String)> = Vec::new();
    for dependency in &generator.dependencies {
        let child_templates = collect_templates(dependency);
        templates.extend(child_templates);
    }
    templates.extend(generator.templates.clone());
    templates
}

fn read_yaml_file<T: for<'de> Deserialize<'de>>(
    base_path: &Path,
    file_name: &str,
) -> Result<T, io::Error> {
    let file_path = base_path.join(file_name);
    let content = fs::read_to_string(file_path.clone()).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Error reading file {file_path:?} due to the following error:{e:?}:"),
        )
    })?;

    let data: T = serde_yaml::from_str(&content).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Cannot deserialize file {file_path:?} due to error:{e:?}"),
        )
    })?;
    Ok(data)
}

fn read_optional_file_as_string(base_path: &Path, file_name: &str) -> Option<String> {
    fs::read_to_string(base_path.join(file_name)).ok()
}

fn read_optional_json_file(base_path: &Path, file_name: &str) -> Option<serde_json::Value> {
    let content = fs::read_to_string(base_path.join(file_name)).ok()?;
    serde_json::from_str(&content).ok()
}

fn read_optional_directory(base_path: &Path, dir_name: &str) -> Option<Vec<String>> {
    let dir_path = base_path.join(dir_name);
    if !dir_path.exists() || !dir_path.is_dir() {
        return None;
    }

    let glob_pattern = base_path.join(dir_name).join("**/*");

    let files: Vec<String> = glob(glob_pattern.to_str().unwrap())
        .unwrap()
        .filter_map(|x| match x {
            Ok(path) if path.is_file() => path.to_str().map(ToString::to_string),
            Ok(_) => None,
            Err(e) => {
                eprintln!("Error: {e:?}");
                None
            }
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();

    if files.is_empty() {
        None
    } else {
        Some(files)
    }
}

fn glob_to_map(pattern: &str) -> HashMap<String, String> {
    glob(pattern)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter_map(|path| {
            let filename = path.file_name()?.to_str()?.to_string();
            let content = fs::read_to_string(&path).ok()?;
            Some((filename, content))
        })
        .collect()
}

fn construct_destination_path(
    base_path: &Path,
    file: &Path,
    destination_dir: &Path,
) -> Result<PathBuf, io::Error> {
    let base_path = base_path.canonicalize().map_err(|e| {
        eprintln!("Error canonicalizing base_path: {e:?}");
        e
    })?;

    let file = file.canonicalize().map_err(|e| {
        eprintln!("Error canonicalizing file: {e:?}");
        e
    })?;

    let destination_file_path = file.strip_prefix(&base_path).map_err(|e| {
        eprintln!("Error stripping prefix from file: {e:?}");
        io::Error::new(io::ErrorKind::Other, "Strip prefix failed")
    })?;
    let destination = destination_dir.join(destination_file_path);
    Ok(destination)
}

fn read_optional_entities(base_path: &Path, dir_name: &str) -> Result<Value, io::Error> {
    let dir_path = base_path.join(dir_name);
    if !dir_path.exists() || !dir_path.is_dir() {
        return Ok(json!({}));
    }

    let files_path = base_path.join(dir_name).join("**/*.json");
    let glob_pattern = files_path.to_str().ok_or_else(|| {
        io::Error::new(
            ErrorKind::InvalidInput,
            "Failed to convert pattern to string",
        )
    })?;

    let schemas: Map<String, Value> = glob(glob_pattern)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        .filter_map(|file_path_result| {
            let file_path = file_path_result.ok()?;
            let content = fs::read_to_string(&file_path).ok()?;
            let schema: Value = serde_json::from_str(&content).ok()?;
            let file_stem = file_path
                .file_stem()?
                .to_str()?
                .trim_end_matches(".schema")
                .to_string();
            Some((file_stem, schema))
        })
        .collect();

    Ok(Value::Object(schemas))
}
