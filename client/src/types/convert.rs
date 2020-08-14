use super::{HttpVerb, Recipe, Rule, RuleType};
use anyhow::{format_err, Error, Result};
use std::convert::TryInto;

impl From<shared::Recipe> for Recipe {
    fn from(r: shared::Recipe) -> Self {
        let shared::Recipe {
            id,
            url,
            payload,
            created_at,
            updated_at,
            rules,
        } = r;
        let rules = rules.into_iter().map(Into::into).collect();
        let payload = payload.to_string();
        Self {
            id,
            url,
            rules,
            payload,
            created_at,
            updated_at,
        }
    }
}

impl TryInto<shared::Recipe> for Recipe {
    type Error = Error;

    fn try_into(self) -> Result<shared::Recipe> {
        let Recipe {
            id,
            url,
            payload,
            created_at,
            updated_at,
            rules,
        } = self;
        let rules = rules
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<shared::Rule>>>()?;
        let payload: serde_json::Value = serde_json::from_str(&payload)?;
        Ok(shared::Recipe {
            id,
            url,
            payload,
            created_at,
            updated_at,
            rules,
        })
    }
}

impl From<shared::Rule> for Rule {
    fn from(r: shared::Rule) -> Self {
        use shared::Rule::*;

        match r {
            Authenticated { key_path, .. } => Rule {
                rule_type: Some(RuleType::Authenticated),
                key_path: Some(key_path),
                ..Rule::default()
            },
            Subject { subject, .. } => Rule {
                rule_type: Some(RuleType::Subject),
                subject: Some(subject),
                ..Rule::default()
            },
            HttpMethod { http_method, .. } => Rule {
                rule_type: Some(RuleType::HttpMethod),
                http_method: Some(http_method.into()),
                ..Rule::default()
            },
        }
    }
}

impl From<shared::HttpVerb> for HttpVerb {
    fn from(s: shared::HttpVerb) -> Self {
        use shared::HttpVerb::*;
        match s {
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

impl TryInto<shared::Rule> for Rule {
    type Error = Error;

    fn try_into(self) -> Result<shared::Rule> {
        let Rule {
            id,
            rule_type,
            key_path,
            subject,
            http_method,
        } = self;
        if let Some(rule_type) = rule_type {
            use RuleType::*;
            Ok(match rule_type {
                Authenticated => shared::Rule::Authenticated {
                    id,
                    key_path: key_path
                        .ok_or_else(|| format_err!("The field, key_path, must be Some!"))?,
                },
                Subject => shared::Rule::Subject {
                    id,
                    subject: subject
                        .ok_or_else(|| format_err!("The field, subject, must be Some!"))?,
                },
                HttpMethod => shared::Rule::HttpMethod {
                    id,
                    http_method: http_method
                        .map(Into::into)
                        .ok_or_else(|| format_err!("The field, http_method, must be Some!"))?,
                },
            })
        } else {
            Err(format_err!("The field, rule_type, must be Some!"))
        }
    }
}
