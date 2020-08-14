use super::{Recipe, Rule, RuleType};
use crate::config::{self, KeyPathKind};
use actix_web::HttpRequest;
use anyhow::{format_err, Context, Result};
use log::debug;
use medallion::{DefaultPayload, DefaultToken};
use serde_json::Value;
use std::{fs, path::PathBuf, str::FromStr};

impl Recipe {
    pub(crate) fn evaluate_rules(
        &self,
        rules: &[Rule],
        request: &HttpRequest,
    ) -> Result<Option<String>> {
        debug!("Evaluating rules for {}", self.url);
        let rules = rules
            .iter()
            .map(|rule| rule.eval(request))
            .inspect(|result| debug!("Result {:?}", result))
            .collect::<Result<Vec<bool>>>()?;
        if rules.iter().all(|rule| *rule) {
            Ok(Some(self.payload.clone()))
        } else {
            Ok(None)
        }
    }
}

impl Rule {
    fn eval(&self, request: &HttpRequest) -> Result<bool> {
        debug!("Evaluating {:?}", self);
        use RuleType::*;

        match &self.rule_type {
            Authenticated => self.is_authenticated(request),
            Subject => self.is_authorized(request),
            HttpMethod => self.is_method(request),
        }
    }

    fn is_authenticated(&self, request: &HttpRequest) -> Result<bool> {
        if let Some(token) = extract_auth_token(request)? {
            debug!("Verifying token {:?}", token);
            let key = match config::KEY_PATH_KIND.clone() {
                KeyPathKind::Directory(mut key_path) => {
                    debug!("KEY PATH={}", key_path.display());
                    let db_key_path = self
                        .key_path
                        .as_ref()
                        .ok_or_else(|| format_err!("Key path was not set!"))?;
                    let db_key_path = PathBuf::from_str(db_key_path.trim_start_matches('/'))
                        .with_context(|| format!("Could not parse {} as a path!", db_key_path))?;
                    key_path.push(db_key_path);
                    debug!("After adding from DB, KEY PATH={}", key_path.display());
                    debug!("Loading key {:?}", key_path.display());
                    fs::read(key_path)?
                }
                KeyPathKind::File(key_path) => {
                    let db_key_path = self
                        .key_path
                        .as_ref()
                        .ok_or_else(|| format_err!("Key path was not set!"))?;
                    debug!("Key reference from database, {}", db_key_path);
                    let key_ref: Vec<String> =
                        db_key_path.split('.').map(ToOwned::to_owned).collect();
                    debug!("Split key ref {:?}", key_ref);
                    debug!("Opening file, {}", key_path.display());
                    let key_data: Value = serde_json::from_reader(fs::File::open(&key_path)?)?;
                    let key = key_ref.iter().fold(Ok(&key_data), |key_data, key_ref| {
                        key_data.and_then(|key_data| {
                            debug!("Key data {:?}", key_data);
                            key_data.get(key_ref).ok_or_else(|| {
                                format_err!("Failed to find property, {}, in key data.", key_ref)
                            })
                        })
                    })?;
                    let key = key.as_str().ok_or_else(|| {
                        format_err!("Key data in the JSON key file wasn't a string!")
                    })?;
                    debug!("Found key, {}", key);
                    key.as_bytes().to_owned()
                }
            };
            token.verify(&key).map_err(anyhow::Error::from)
        } else {
            Ok(false)
        }
    }

    fn is_authorized(&self, request: &HttpRequest) -> Result<bool> {
        if let Some(token) = extract_auth_token(request)? {
            debug!("Authorizing token {:?}", token);
            Ok(token.payload.sub == self.subject)
        } else {
            Ok(false)
        }
    }

    fn is_method(&self, _request: &HttpRequest) -> Result<bool> {
        todo!("Evaluate method rule")
    }
}

fn extract_auth_token(request: &HttpRequest) -> Result<Option<DefaultToken<DefaultPayload>>> {
    let auth = request.headers().get("Authorization");
    if let Some(auth) = auth {
        let auth = auth.to_str()?;
        let auth = auth.trim_start_matches("Bearer").trim();
        let auth = auth.split('.').take(3).collect::<Vec<&str>>().join(".");
        DefaultToken::parse(&auth)
            .map(Some)
            .map_err(anyhow::Error::from)
    } else {
        Ok(None)
    }
}
