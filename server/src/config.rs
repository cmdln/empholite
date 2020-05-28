use anyhow::{bail, format_err, Context, Result};
use lazy_static::lazy_static;
use std::{env, path::PathBuf};

const CLIENT_PATH: &str = "client path";
const STATIC_PATH: &str = "static path";

lazy_static! {
    pub(crate) static ref FAVICON: String =
        file_from_opt_env(STATIC_PATH, "favicon.svg", "./static/", &["static"],)
            .unwrap_or_else(|error| panic!("{}", error));
    pub(crate) static ref WASM: String =
        file_from_opt_env(CLIENT_PATH, "client_bg.wasm", "./client/pkg/", &["client"],)
            .unwrap_or_else(|error| panic!("{}", error));
}

pub(crate) struct ServerConfig {
    pub(crate) bind_address: String,
    pub(crate) client_bundle_path: String,
    pub(crate) static_file_path: String,
}

pub(crate) fn init() {
    let _ = *FAVICON;
    let _ = *WASM;
}

pub(crate) fn server_config() -> Result<ServerConfig> {
    let host = from_opt_env("host", "0.0.0.0");
    let port: i32 = from_opt_env("port", "8989").parse()?;
    let client_bundle_path = path_from_opt_env(CLIENT_PATH, "./client/pkg/", &["client"])?;
    let static_file_path = path_from_opt_env(STATIC_PATH, "./static", &["static"])?;
    let bind_address = format!("{}:{}", host, port);
    Ok(ServerConfig {
        bind_address,
        client_bundle_path,
        static_file_path,
    })
}

fn from_opt_env(option_name: &str, default: &str) -> String {
    env::var(option_name).unwrap_or_else(|_| String::from(default))
}

fn path_from_opt_env(option_name: &str, under_cargo: &str, subdirs: &[&str]) -> Result<String> {
    env::var(option_name)
        .with_context(|| {
            format!(
                "Unable to get {} from command line or environment",
                option_name
            )
        })
        .or_else(|_| default_path(under_cargo, subdirs))
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

fn file_from_opt_env(
    option_name: &str,
    filename: &str,
    under_cargo: &str,
    subdirs: &[&str],
) -> Result<String> {
    path_from_opt_env(option_name, under_cargo, subdirs)
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

fn default_path(under_cargo: &str, subdirs: &[&str]) -> Result<String> {
    if env::var("CARGO").is_ok() {
        Ok(String::from(under_cargo))
    } else {
        let mut location = PathBuf::from("/");
        for subdir in subdirs {
            location.push(subdir);
        }
        if !location.exists() {
            bail!("The default path, {}, doesn't exist!", location.display())
        }
        Ok(format!("{}", location.display()))
    }
}
