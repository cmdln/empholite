use anyhow::{format_err, Context, Result};
use lazy_static::lazy_static;
use std::{convert::TryFrom, env, path::PathBuf, str::FromStr};

const CLIENT_PATH: &str = "CLIENT_PATH";
const STATIC_PATH: &str = "STATIC_PATH";

lazy_static! {
    pub(crate) static ref FAVICON: String = file_from_env_or_default(
        STATIC_PATH,
        "favicon.svg",
        default_path("./static/", &["static"])
    )
    .unwrap_or_else(|error| panic!("{}", error));
    pub(crate) static ref WASM: String = file_from_env_or_default(
        CLIENT_PATH,
        "client_bg.wasm",
        default_path("./client/pkg/", &["client"])
    )
    .unwrap_or_else(|error| panic!("{}", error));
    pub(crate) static ref KEY_PATH_KIND: KeyPathKind =
        key_path_kind().unwrap_or_else(|error| panic!("{}", error));
}

#[derive(Debug, Clone)]
pub(crate) enum KeyPathKind {
    Directory(PathBuf),
    File(PathBuf, Vec<String>),
}

impl TryFrom<String> for KeyPathKind {
    type Error = anyhow::Error;

    fn try_from(key_path_var: String) -> Result<KeyPathKind> {
        match key_path_var.to_ascii_uppercase().as_str() {
            "directory" | "dir" => Ok(KeyPathKind::Directory(key_path()?)),
            "file" => Ok(KeyPathKind::File(key_path()?, key_ref()?)),
            _ => Err(format_err!(
                "{} cannot be parsed as a kind of key path!",
                key_path_var
            )),
        }
    }
}

pub(crate) struct ServerConfig {
    pub(crate) bind_address: String,
    pub(crate) database_url: String,
    pub(crate) client_bundle_path: String,
    pub(crate) static_file_path: String,
}

pub(crate) fn init() {
    let _ = *FAVICON;
    let _ = *WASM;
}

pub(crate) fn server_config() -> Result<ServerConfig> {
    let host = env_or_default("host", "0.0.0.0");
    let port: i32 = env_or_default("port", "8989").parse()?;
    let client_bundle_path =
        path_from_env_or_default(CLIENT_PATH, default_path("./client/pkg/", &["client"]))?;
    let static_file_path =
        path_from_env_or_default(STATIC_PATH, default_path("./static", &["static"]))?;
    let bind_address = format!("{}:{}", host, port);
    let database_url = env::var("DATABASE_URL").with_context(|| "DATABASE_URL is not set!")?;
    Ok(ServerConfig {
        bind_address,
        database_url,
        client_bundle_path,
        static_file_path,
    })
}

fn key_path() -> Result<PathBuf> {
    env::var("KEY_PATH")
        .map_err(anyhow::Error::from)
        .and_then(|key_path| {
            PathBuf::from_str(&key_path)
                .map_err(anyhow::Error::from)
                .with_context(|| "Could not parse $KEY_PATH as a path!")
        })
        .or_else(|_| {
            env::var("HOME")
                .map_err(anyhow::Error::from)
                .with_context(|| {
                    "$HOME does not appear to be set, necessary when $KEY_PATH is not set!"
                })
                .and_then(|home| {
                    PathBuf::from_str(&home)
                        .map_err(anyhow::Error::from)
                        .with_context(|| "Could not parse $HOME as a path!")
                })
                .map(|mut home| {
                    home.push(".digital-auth-keys");
                    home.push("qa");
                    home.push("keys");
                    home
                })
        })
}

fn key_ref() -> Result<Vec<String>> {
    env::var("KEY_REF")
        .map_err(anyhow::Error::from)
        .map(|key_path| key_path.split('.').map(ToOwned::to_owned).collect())
}

fn key_path_kind() -> Result<KeyPathKind> {
    env::var("KEY_PATH_KIND")
        .map_err(anyhow::Error::from)
        .and_then(TryFrom::try_from)
        .or_else(|_| Ok(KeyPathKind::Directory(key_path()?)))
}

fn env_or_default(option_name: &str, default: &str) -> String {
    env::var(option_name).unwrap_or_else(|_| String::from(default))
}

fn path_from_env_or_default(option_name: &str, default_path: String) -> Result<String> {
    env::var(option_name)
        .with_context(|| {
            format!(
                "Unable to get {} from command line or environment",
                option_name
            )
        })
        .or_else(|_| Ok(default_path))
        .and_then(|path| {
            if PathBuf::from(&path).exists() {
                Ok(path)
            } else {
                Err(format_err!(
                    "Path for {}, {}, does not exist!",
                    option_name,
                    path
                ))
            }
        })
}

fn file_from_env_or_default(
    option_name: &str,
    filename: &str,
    default_path: String,
) -> Result<String> {
    path_from_env_or_default(option_name, default_path)
        .map(|path| {
            if path.ends_with(filename) {
                path
            } else {
                format!("{}/{}", path, filename)
            }
        })
        .and_then(|path| {
            if PathBuf::from(&path).exists() {
                Ok(path)
            } else {
                Err(format_err!(
                    "Path to {}, {}, does not exist!",
                    filename,
                    path
                ))
            }
        })
}

fn default_path(under_cargo: &str, subdirs: &[&str]) -> String {
    if env::var("CARGO").is_ok() {
        String::from(under_cargo)
    } else {
        let mut location = PathBuf::from("/");
        for subdir in subdirs {
            location.push(subdir);
        }
        format!("{}", location.display())
    }
}
