mod convert;

use chrono::NaiveDateTime;
use http::Uri;
use log::error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Default, Serialize, Deserialize, Debug, Validate)]
pub(crate) struct Recipe {
    pub(crate) id: Option<Uuid>,
    #[validate(custom(
        function = "url_starts_with_api",
        message = "The endpoint must be a valid URL that includes a path that starts with \"/api\""
    ))]
    pub(crate) url: String,
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
