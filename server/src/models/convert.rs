use super::{Recipe, RecipeCascaded, Rule, RuleType};
use anyhow::{format_err, Error, Result};
use std::convert::TryInto;

impl Into<shared::Recipe> for Recipe {
    fn into(self) -> shared::Recipe {
        let Recipe {
            url,
            payload,
            id,
            created_at,
            updated_at,
        } = self;
        let id = Some(id);
        let created_at = Some(created_at);
        let updated_at = Some(updated_at);
        // TODO convert rule elements
        let rules = Vec::new();
        shared::Recipe {
            id,
            url,
            rules,
            payload,
            created_at,
            updated_at,
        }
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
            ..
        } = self;
        Ok(match rule_type {
            Authenticated => shared::Rule::Authenticated(
                key_path.ok_or_else(|| format_err!("Field, key_path, must be Some!"))?,
            ),
            Subject => shared::Rule::Subject(
                subject.ok_or_else(|| format_err!("Field, subject, must be Some!"))?,
            ),
        })
    }
}
