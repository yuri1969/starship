use clap::ArgMatches;
use lazy_static::lazy_static;
use starship;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::{io, process};

lazy_static! {
    static ref MANIFEST_DIR: &'static Path = Path::new(env!("CARGO_MANIFEST_DIR"));
    pub static ref FIXTURES_DIR: PathBuf = MANIFEST_DIR.join("tests/fixtures");
    static ref EMPTY_CONFIG: PathBuf = MANIFEST_DIR.join("empty_config.toml");
}

/// Render the full starship prompt
pub fn render_prompt() -> process::Command {
    let mut command = process::Command::new("./target/debug/starship");

    command
        .arg("prompt")
        .env_clear()
        .env("PATH", env!("PATH")) // Provide the $PATH variable so that external programs are runnable
        .env("STARSHIP_CONFIG", EMPTY_CONFIG.as_os_str());

    command
}

pub fn render_module(module_name: &str) -> RenderModule {
    RenderModule::new(module_name)
}

struct RenderModule {
    module_name: String,
    args: Vec<String>,
    env: std::collections::HashMap<String, String>,
}

impl RenderModule {
    fn new(module_name: &str) -> Self {
        RenderModule {
            module_name: module_name.to_owned(),
            args: Vec::new(),
            env: std::collections::HashMap::default(),
        }
    }

    pub fn arg<T>(&self, arg: T) -> &Self
    where
        T: Into<String>,
    {
        self.args.push(arg.into());
        self
    }

    pub fn path(&self, path: &std::path::Path) -> &Self {
        self.arg(format!("--path={:?}", path.as_os_str()));
        self
    }

    pub fn env<T>(&self, key: T, value: T) -> &Self 
    where
        T: Into<String>,
    {
        self.env.insert(key.into(), value.into());
        self
    }

    pub fn output(&self) -> Option<String> {
        let app = starship::app::app();
        let args = app.get_matches_from(self.args);
        let context = starship::context::Context::new(args);
        
        let module = starship::modules::handle(self.module_name.as_ref(), &context);
        module.map(|module| module.to_string())
    }
}

//     pub fn set_args(&self, arguments: &str) -> Self {
//         self.args =
//     }
// }

// /// Render a specific starship module by name
// pub fn render_module(module_name: &str) -> process::Command {
//     let args = clap::ArgMatches::default();

//     let context = Context::new_with_dir(args, &dir.into_path());
//     let actual = modules::handle("python", &context).unwrap().to_string();

//     let mut command = process::Command::new("./target/debug/starship");

//     command
//         .arg("module")
//         .arg(module_name)
//         .env_clear()
//         .env("PATH", env!("PATH")) // Provide the $PATH variable so that external programs are runnable
//         .env("STARSHIP_CONFIG", EMPTY_CONFIG.as_os_str());

//     command
// }

// pub fn new_render_module(module_name: &str) -> String
//     where
//         T: Into<PathBuf>,
//     {
//     let args = clap::ArgMatches::default();
//     new_render_module_with_args(module_name, args);
// }

// pub fn new_render_module_with_args(module_name: &str) -> String {
//     let args = clap::ArgMatches::default();
//     let context = Context::new_with_dir(args, &dir.into_path());

//     modules::handle("python", &context).unwrap().to_string();
// }

// pub fn new_render_module(module_name: &str, arguments: &str) -> String {
//     print::module(module_name);
// }

/// Create a temporary directory with full access permissions (rwxrwxrwt).
pub fn new_tempdir() -> io::Result<tempfile::TempDir> {
    //  Using `tempfile::TempDir` directly creates files on macOS within
    // "/var/folders", which provides us with restricted permissions (rwxr-xr-x)
    tempfile::tempdir_in("/tmp")
}

/// Extends `std::process::Command` with methods for testing
pub trait TestCommand {
    fn use_config(&mut self, toml: toml::value::Value) -> &mut process::Command;
}

impl TestCommand for process::Command {
    /// Create a configuration file with the provided TOML and use it
    fn use_config(&mut self, toml: toml::value::Value) -> &mut process::Command {
        // Create a persistent config file in a tempdir
        let (mut config_file, config_path) =
            tempfile::NamedTempFile::new().unwrap().keep().unwrap();
        write!(config_file, "{}", toml.to_string()).unwrap();

        // Set that newly-created file as the config for the prompt instance
        self.env("STARSHIP_CONFIG", config_path)
    }
}
