use ansi_term::Color;
use std::ffi::OsStr;
use std::iter::Iterator;
use std::ops::Deref;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;

use super::{Context, Module};

type JValue = serde_json::Value;

const DOTNET_SYMBOL: &str = "•NET ";
const GLOBAL_JSON_FILE: &str = "global.json";
const PROJECT_JSON_FILE: &str = "project.json";

/// A module which shows the latest (or pinned) version of the dotnet SDK
///
/// Will display if any of the following file extensions are present in
/// the current directory: .sln, .csproj, .fsproj, .xproj
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let dotnet_files = get_local_dotnet_files(context).ok()?;
    if dotnet_files.is_empty() {
        return None;
    }

    let mut module = context.new_module("dotnet");
    let enable_heuristic = module.config_value_bool("heuristic").unwrap_or(true);

    let version = if enable_heuristic {
        let repo_root = context
            .get_repo()
            .ok()
            .and_then(|r| r.root.as_ref().map(|buf| buf.as_path()));
        estimate_dotnet_version(&dotnet_files, &context.current_dir, repo_root)?
    } else {
        get_version_from_cli()?
    };

    module.set_style(Color::Blue.bold());
    module.new_segment("symbol", DOTNET_SYMBOL);
    module.new_segment("version", &version);

    Some(module)
}

fn estimate_dotnet_version<'a>(
    files: &'a Vec<DotNetFile<'a>>,
    current_dir: &Path,
    repo_root: Option<&Path>,
) -> Option<Version> {
    let get_file_of_type = |t: FileType| files.iter().find(|f| f.file_type == t);

    // Check for a global.json or a solution file first, but otherwise take the first file
    let relevant_file = get_file_of_type(FileType::GlobalJson)
        .or_else(|| get_file_of_type(FileType::SolutionFile))
        .or_else(|| files.iter().next())?;

    match relevant_file.file_type {
        FileType::GlobalJson => {
            get_pinned_sdk_version_from_file(relevant_file.path).or_else(get_latest_sdk_from_cli)
        }
        FileType::SolutionFile => {
            // With this heuristic, we'll assume that a "global.json" won't
            // be found in any directory above the solution file.
            get_latest_sdk_from_cli()
        }
        FileType::ProjectFile | FileType::ProjectJson => {
            let mut check_dirs = current_dir
                .parent()
                .iter()
                .chain(repo_root.iter())
                .map(|d| *d)
                .collect::<Vec<&Path>>();
            check_dirs.dedup();

            check_dirs
                .iter()
                .filter_map(|d| check_directory_for_global_json(d))
                // This should lazily evaluate the first directory with a global.json
                .next()
                .or_else(get_latest_sdk_from_cli)
        }
    }
}

fn check_directory_for_global_json(path: &Path) -> Option<Version> {
    let global_json_path = path.join(GLOBAL_JSON_FILE);
    if global_json_path.exists() {
        get_pinned_sdk_version_from_file(&global_json_path)
    } else {
        None
    }
}

fn get_pinned_sdk_version_from_file(path: &Path) -> Option<Version> {
    let json_text = crate::utils::read_file(path).ok()?;
    get_pinned_sdk_version(&json_text)
}

fn get_pinned_sdk_version(json: &str) -> Option<Version> {
    let parsed_json: JValue = serde_json::from_str(json).ok()?;

    match parsed_json {
        JValue::Object(root) => {
            let sdk = root.get("sdk")?;
            match sdk {
                JValue::Object(sdk) => {
                    let version = sdk.get("version")?;
                    match version {
                        JValue::String(version_string) => Some(Version(version_string.clone())),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn get_local_dotnet_files<'a>(context: &'a Context) -> Result<Vec<DotNetFile<'a>>, std::io::Error> {
    Ok(context
        .get_dir_files()?
        .iter()
        .filter_map(|p| {
            get_dotnet_file_type(p).map(|t| DotNetFile {
                path: p.as_ref(),
                file_type: t,
            })
        })
        .collect())
}

fn get_dotnet_file_type(path: &Path) -> Option<FileType> {
    let file_name_lower = map_str_to_lower(path.file_name());

    match file_name_lower.as_ref().map(|f| f.as_ref()) {
        Some(GLOBAL_JSON_FILE) => return Some(FileType::GlobalJson),
        Some(PROJECT_JSON_FILE) => return Some(FileType::ProjectJson),
        _ => (),
    };

    let extension_lower = map_str_to_lower(path.extension());

    match extension_lower.as_ref().map(|f| f.as_ref()) {
        Some("sln") => return Some(FileType::SolutionFile),
        Some("csproj") | Some("fsproj") | Some("xproj") => return Some(FileType::ProjectFile),
        _ => (),
    };

    None
}

fn map_str_to_lower(value: Option<&OsStr>) -> Option<String> {
    Some(value?.to_str()?.to_ascii_lowercase())
}

fn get_version_from_cli() -> Option<Version> {
    let version_output = Command::new("dotnet").arg("--version").output().ok()?;
    let version = str::from_utf8(version_output.stdout.as_slice())
        .ok()?
        .trim();

    let mut buffer = String::with_capacity(version.len() + 1);
    buffer.push('v');
    buffer.push_str(version);

    Some(Version(buffer))
}

fn get_latest_sdk_from_cli() -> Option<Version> {
    let mut cmd = Command::new("dotnet");
    cmd.arg("--list-sdks")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .stdin(Stdio::null());
    let exit_code = cmd.status().ok()?;

    if exit_code.success() {
        let sdks_output = cmd.output().ok()?;
        let latest_sdk = str::from_utf8(sdks_output.stdout.as_slice())
            .ok()?
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .last()?;
        let take_until = latest_sdk.find('[')? - 1;
        if take_until > 1 {
            let version = &latest_sdk[..take_until];
            let mut buffer = String::with_capacity(version.len() + 1);
            buffer.push('v');
            buffer.push_str(version);
            Some(Version(buffer))
        } else {
            None
        }
    } else {
        // Older versions of the dotnet cli do not support the --list-sdks command
        // So, if the status code indicates failure, fall back to `dotnet --version`
        // TODO: Log this
        get_version_from_cli()
    }
}

struct DotNetFile<'a> {
    path: &'a Path,
    file_type: FileType,
}

#[derive(PartialEq)]
enum FileType {
    ProjectJson,
    ProjectFile,
    GlobalJson,
    SolutionFile,
}

struct Version(String);

impl Deref for Version {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn should_parse_version_from_global_json() {
    let json_text = r#"
        {
            "sdk": {
                "version": "1.2.3"
            }
        }
    "#;

    let version = get_pinned_sdk_version(json_text).unwrap();
    assert_eq!("1.2.3", version.0);
}

#[test]
fn should_ignore_empty_global_json() {
    let json_text = "{}";

    let version = get_pinned_sdk_version(json_text);
    assert!(version.is_none());
}
