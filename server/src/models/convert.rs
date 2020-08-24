use super::{HttpVerb, NewRule, Recipe, RecipeCascaded, Rule, RuleType};
use anyhow::{bail, format_err, Error, Result};
use serde_json::Value;
use std::convert::{TryFrom, TryInto};
use uuid::Uuid;

impl TryInto<shared::Recipe> for Recipe {
    type Error = Error;

    fn try_into(self) -> Result<shared::Recipe> {
        let Recipe {
            url,
            payload,
            id,
            created_at,
            updated_at,
        } = self;
        let id = Some(id);
        let payload = serde_json::from_str(&payload)?;
        let created_at = Some(created_at);
        let updated_at = Some(updated_at);
        Ok(shared::Recipe {
            id,
            url,
            payload,
            created_at,
            updated_at,
            ..shared::Recipe::default()
        })
    }
}

impl TryInto<shared::Recipe> for RecipeCascaded {
    type Error = Error;

    fn try_into(self) -> Result<shared::Recipe> {
        let Recipe {
            id,
            url,
            payload,
            created_at,
            updated_at,
        } = self.0;
        let rules = self
            .1
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<shared::Rule>>>()?;
        let id = Some(id);
        let payload: Value = serde_json::from_str(&payload)?;
        let created_at = Some(created_at);
        let updated_at = Some(updated_at);
        Ok(shared::Recipe {
            id,
            url,
            rules,
            payload,
            created_at,
            updated_at,
        })
    }
}

impl TryInto<shared::Rule> for Rule {
    type Error = Error;

    fn try_into(self) -> Result<shared::Rule> {
        use RuleType::*;
        let Rule {
            rule_type,
            key_path,
            subject,
            http_method,
            id,
            ..
        } = self;
        let id = Some(id);
        Ok(match rule_type {
            Authenticated => shared::Rule::Authenticated {
                id,
                key_path: key_path.ok_or_else(|| format_err!("Field, key_path, must be Some!"))?,
            },
            Subject => shared::Rule::Subject {
                id,
                subject: subject.ok_or_else(|| format_err!("Field, subject, must be Some!"))?,
            },
            HttpMethod => shared::Rule::HttpMethod {
                id,
                http_method: http_method
                    .map(Into::into)
                    .ok_or_else(|| format_err!("Field, http_method, must be Some!"))?,
            },
        })
    }
}

impl TryFrom<(Uuid, shared::Rule)> for Rule {
    type Error = Error;
    fn try_from(t: (Uuid, shared::Rule)) -> Result<Self> {
        use shared::Rule::*;
        let (recipe_id, r) = t;
        Ok(match r {
            Authenticated { id, key_path } => {
                let id = id.ok_or_else(|| format_err!("Rule must have an ID!"))?;
                Self {
                    id,
                    recipe_id,
                    rule_type: RuleType::Authenticated,
                    subject: None,
                    key_path: Some(key_path),
                    http_method: None,
                }
            }
            Subject { id, subject } => {
                let id = id.ok_or_else(|| format_err!("Rule must have an ID!"))?;
                Self {
                    id,
                    recipe_id,
                    rule_type: RuleType::Subject,
                    subject: Some(subject),
                    key_path: None,
                    http_method: None,
                }
            }
            HttpMethod { id, http_method } => {
                let id = id.ok_or_else(|| format_err!("Rule must have an ID!"))?;
                Self {
                    id,
                    recipe_id,
                    rule_type: RuleType::HttpMethod,
                    subject: None,
                    key_path: None,
                    http_method: Some(http_method.into()),
                }
            }
        })
    }
}

impl From<(Uuid, shared::Rule)> for NewRule {
    fn from(t: (Uuid, shared::Rule)) -> Self {
        let (recipe_id, r) = t;
        use shared::Rule::*;
        match r {
            Authenticated { key_path, .. } => Self {
                recipe_id,
                rule_type: RuleType::Authenticated,
                subject: None,
                key_path: Some(key_path),
                http_method: None,
            },
            Subject { subject, .. } => Self {
                recipe_id,
                rule_type: RuleType::Subject,
                subject: Some(subject),
                key_path: None,
                http_method: None,
            },
            HttpMethod { http_method, .. } => Self {
                recipe_id,
                rule_type: RuleType::HttpMethod,
                subject: None,
                key_path: None,
                http_method: Some(http_method.into()),
            },
        }
    }
}

impl From<shared::HttpVerb> for HttpVerb {
    fn from(v: shared::HttpVerb) -> Self {
        use shared::HttpVerb::*;
        match v {
            Get => HttpVerb::Get,
            Post => HttpVerb::Post,
            Put => HttpVerb::Put,
            Delete => HttpVerb::Delete,
        }
    }
}

impl Into<shared::HttpVerb> for HttpVerb {
    fn into(self) -> shared::HttpVerb {
        use HttpVerb::*;
        match self {
            Get => shared::HttpVerb::Get,
            Post => shared::HttpVerb::Post,
            Put => shared::HttpVerb::Put,
            Delete => shared::HttpVerb::Delete,
        }
    }
}

impl TryFrom<&str> for HttpVerb {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        use HttpVerb::*;
        match s {
            "Get" => Ok(Get),
            "Post" => Ok(Post),
            "Put" => Ok(Put),
            "Delete" => Ok(Delete),
            _ => bail!(
                "{} is not a valid HTTP verb! For conversion from strings, case matters.",
                s
            ),
        }
    }
}

impl TryFrom<&str> for RuleType {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        use RuleType::*;
        match s {
            "Authenticated" => Ok(Authenticated),
            "Subject" => Ok(Subject),
            "HttpMethod" => Ok(HttpMethod),
            _ => bail!("{} is not a valid rule type!", s),
        }
    }
}
