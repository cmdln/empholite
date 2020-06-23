mod convert;

use chrono::NaiveDateTime;
use http::Uri;
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub(super) enum RuleType {
    Authenticated,
    Subject,
}

#[derive(Default, Debug, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_rule"))]
pub(super) struct Rule {
    pub(super) rule_type: Option<RuleType>,
    pub(super) subject: Option<String>,
    pub(super) key_path: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub(crate) struct Recipe {
    pub(crate) id: Option<Uuid>,
    #[validate(custom(
        function = "url_starts_with_api",
        message = "The endpoint must be a valid URL that includes a path that starts with \"/api\""
    ))]
    pub(crate) url: String,
    #[validate]
    pub(crate) rules: Vec<Rule>,
    #[validate(custom(function = "payload_is_json", message = "Payload must be valid JSON!"))]
    pub(crate) payload: String,
    pub(crate) created_at: Option<NaiveDateTime>,
    pub(crate) updated_at: Option<NaiveDateTime>,
}

fn payload_is_json(payload: &str) -> Result<(), ValidationError> {
    if let Err(error) = serde_json::from_str::<serde_json::Value>(payload) {
        error!("Payload could not be parsed as JSON, {}", error);
        Err(ValidationError::new("invalid_payload"))
    } else {
        Ok(())
    }
}

fn url_starts_with_api(url: &str) -> Result<(), ValidationError> {
    let uri: Uri = url
        .parse()
        .map_err(|_| ValidationError::new("invalid_url"))?;
    if let Some(path_and_query) = uri.path_and_query() {
        if !path_and_query.as_str().starts_with("/api") {
            Err(ValidationError::new("path_missing_api"))
        } else {
            Ok(())
        }
    } else {
        Err(ValidationError::new("path_missing"))
    }
}

fn validate_rule(r: &Rule) -> Result<(), ValidationError> {
    use RuleType::*;

    match r {
        Rule {
            rule_type: Some(Authenticated),
            key_path: None,
            ..
        } => Err(ValidationError {
            code: "invalid_authenticated_rule".into(),
            message: Some("The key path is required to check that a call is authenticated!".into()),
            params: HashMap::new(),
        }),
        Rule {
            rule_type: Some(Subject),
            subject: None,
            ..
        } => Err(ValidationError {
            code: "invalid_subject_rule".into(),
            message: Some(
                "The subject claim is required to check that a call has a specific subject!".into(),
            ),
            params: HashMap::new(),
        }),
        Rule {
            rule_type: None, ..
        } => Err(ValidationError {
            code: "rule_type_required".into(),
            message: Some("".into()),
            params: HashMap::new(),
        }),
        _ => Ok(()),
    }
}
