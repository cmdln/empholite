use super::db;
use crate::config::{self, KeyPathKind};
use actix_web::{error::ErrorInternalServerError, web::Path, HttpResponse, Result};
use anyhow::{bail, format_err};
use serde_json::{self, Map, Value};
use std::{fs, path::PathBuf};

mod recipe;

pub(crate) use recipe::*;

#[actix_web::get("/ajax/key_path/{tail:.*}")]
pub(crate) async fn complete_key_path(selected_path: Path<String>) -> Result<HttpResponse> {
    use log::debug;
    debug!("Key path: {:?}", selected_path);
    let selected_path = selected_path.into_inner();
    let selected: Vec<String> = if selected_path.is_empty() {
        Vec::new()
    } else {
        into_selected(selected_path)
    };
    debug!("Parsed: {:?}", selected);
    let entries = match config::KEY_PATH_KIND.clone() {
        KeyPathKind::Directory(mut key_path) => {
            directory_completions(selected, &mut key_path).map_err(ErrorInternalServerError)?
        }
        KeyPathKind::File(ref key_path) => {
            property_completions(selected, &key_path).map_err(ErrorInternalServerError)?
        }
    };
    Ok(HttpResponse::Ok().json(entries))
}

#[actix_web::get("/ajax/config")]
pub(crate) async fn get_config() -> Result<HttpResponse> {
    let config = shared::Config {
        key_path_kind: if let KeyPathKind::Directory(_) = *config::KEY_PATH_KIND {
            shared::KeyPathKind::Directory
        } else {
            shared::KeyPathKind::File
        },
    };
    Ok(HttpResponse::Ok().json(config))
}

fn directory_completions(
    selected: Vec<String>,
    key_path: &mut PathBuf,
) -> anyhow::Result<shared::KeyPathCompletions> {
    let selected_path = PathBuf::from(selected.join("/"));
    key_path.push(selected_path);
    let entries = fs::read_dir(key_path)?;
    let candidates: anyhow::Result<Vec<shared::KeyPathComponent>> = entries
        .map(|entry| {
            let entry = entry?;
            let leaf = !entry.path().is_dir();
            entry
                .path()
                .file_name()
                .ok_or_else(|| {
                    format_err!(
                        "Could not determine filename for {}",
                        entry.path().display()
                    )
                })
                .map(|os_str| os_str.to_os_string())
                .and_then(|os_string| {
                    os_string.into_string().map_err(|error| {
                        format_err!("Filename contained a non-Unicode character! {:?}", error)
                    })
                })
                .map(|component| shared::KeyPathComponent { leaf, component })
        })
        .collect();

    let candidates = candidates?;

    Ok(shared::KeyPathCompletions {
        candidates,
        selected,
        kind: shared::KeyPathKind::Directory,
    })
}

fn property_completions(
    selected: Vec<String>,
    key_path: &PathBuf,
) -> anyhow::Result<shared::KeyPathCompletions> {
    let key_file = fs::read_to_string(key_path)?;
    let key_material: Value = serde_json::from_str(&key_file)?;
    let key_ref = selected.clone();
    if key_ref.is_empty() {
        let key_material = key_material.as_object().ok_or_else(|| {
            format_err!(
                "The root of the JSON Key file, {}, must be an object!",
                key_path.display()
            )
        })?;
        Ok(into_completions(&key_material, selected))
    } else {
        let nested = key_ref.iter().fold(Ok(&key_material), |nested, prop| {
            nested.and_then(|nested| {
                nested.get(prop).ok_or_else(|| {
                    format_err!("\"{}\" is not a valid key reference!", key_ref.join("."))
                })
            })
        })?;
        if let Value::Object(nested) = nested {
            Ok(into_completions(nested, selected))
        } else {
            bail!("\"{}\" is not a valid key reference!", key_ref.join("."))
        }
    }
}

fn into_selected(selected_path: String) -> Vec<String> {
    let separator = if selected_path.contains('/') {
        '/'
    } else {
        '/'
    };
    selected_path
        .split(separator)
        .map(ToOwned::to_owned)
        .filter(|c| !c.trim().is_empty())
        .collect()
}

fn into_completions(
    key_material: &Map<String, Value>,
    selected: Vec<String>,
) -> shared::KeyPathCompletions {
    let candidates = key_material
        .iter()
        .map(|(key, value)| shared::KeyPathComponent {
            leaf: !value.is_object(),
            component: key.clone(),
        })
        .collect();
    shared::KeyPathCompletions {
        candidates,
        selected,
        kind: shared::KeyPathKind::File,
    }
}
