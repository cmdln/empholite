use super::{Recipe, Rule, RuleType};
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
            Authenticated(key_path) => Rule {
                rule_type: Some(RuleType::Authenticated),
                key_path: Some(key_path),
                ..Rule::default()
            },
            Subject(subject) => Rule {
                rule_type: Some(RuleType::Authenticated),
                subject: Some(subject),
                ..Rule::default()
            },
        }
    }
}

impl TryInto<shared::Rule> for Rule {
    type Error = Error;

    fn try_into(self) -> Result<shared::Rule> {
        let Rule {
            rule_type,
            key_path,
            subject,
        } = self;
        if let Some(rule_type) = rule_type {
            use RuleType::*;
            Ok(match rule_type {
                Authenticated => shared::Rule::Authenticated(
                    key_path.ok_or_else(|| format_err!("The field, key_path, must be Some!"))?,
                ),
                Subject => shared::Rule::Subject(
                    subject.ok_or_else(|| format_err!("The field, subject, must be Some!"))?,
                ),
            })
        } else {
            Err(format_err!("The field, rule_type, must be Some!"))
        }
    }
}
