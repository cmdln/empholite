use super::{Recipe, Rule, RuleType};
use actix_web::HttpRequest;
use anyhow::{format_err, Context, Result};
use log::debug;
use medallion::{DefaultPayload, DefaultToken};
use std::{path::PathBuf, str::FromStr};

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

        match self.rule_type {
            Authenticated => self.is_authenticated(request),
            Subject => self.is_authorized(request),
        }
    }

    fn is_authenticated(&self, request: &HttpRequest) -> Result<bool> {
        if let Some(token) = extract_auth_token(request)? {
            debug!("Verifying token {:?}", token);
            let mut key_path = crate::config::KEY_PATH.clone();
            debug!("KEY PATH={}", key_path.display());
            let db_key_path = self
                .key_path
                .as_ref()
                .ok_or_else(|| format_err!("Key path was not set!"))?;
            let db_key_path = PathBuf::from_str(db_key_path.trim_start_matches('/'))
                .with_context(|| format!("Could not parse {} as a path!", db_key_path))?;
            key_path.push(db_key_path);
            debug!("After adding from DB, KEY PATH={}", key_path.display());
            use std::fs;
            debug!("Loading key {:?}", key_path.display());
            let key = fs::read(key_path)?;
            debug!("Loaded key {:?}", self.key_path);
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
