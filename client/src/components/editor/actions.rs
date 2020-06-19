use super::{Editor, Msg};
use crate::components::alert::Context;
use anyhow::{bail, format_err, Context as _, Result};
use http::Uri;
use log::{debug, error, trace};
use yew::{
    format::{Nothing, Text},
    prelude::*,
    services::fetch::{Request, Response, StatusCode},
};

impl Editor {
    pub(super) fn handle_fetch(&mut self) -> Result<ShouldRender> {
        debug!("Recipe {:?}", self.state);
        let request = Request::get(format!(
            "/ajax/recipe/{}",
            self.props
                .id
                .ok_or_else(|| format_err!("Cannot fetch recipe, ID is not set!"))?
        ))
        .body(Nothing)
        .map_err(anyhow::Error::from)?;
        let task = self.fetch_svc.fetch(
            request,
            self.link.callback(
                move |response: Response<Text>| match response.into_parts() {
                    (meta, Ok(body)) if meta.status >= StatusCode::BAD_REQUEST => {
                        Msg::Failure(body)
                    }
                    (_, Ok(body)) => Msg::Fetched(body),
                    (_, Err(error)) => {
                        error!("{}", error);
                        Msg::Failure(format!("{}", error))
                    }
                },
            ),
        )?;
        self.fetch_tsk = Some(task);
        Ok(false)
    }

    pub(super) fn handle_fetched(&mut self, body: String) -> Result<ShouldRender> {
        let state: shared::Recipe = serde_json::from_str(&body)
            .with_context(|| "Error parsing JSON when trying to fetch a recipe!")?;
        self.state = state;
        self.fetch_tsk = None;
        Ok(true)
    }

    pub(super) fn handle_url_change(&mut self, url: String) -> Result<ShouldRender> {
        self.state.url = url;
        Ok(true)
    }

    pub(super) fn handle_payload_change(&mut self, payload: String) -> Result<ShouldRender> {
        self.state.payload = payload;
        Ok(true)
    }

    pub(super) fn handle_failure(&mut self, error: String) -> Result<ShouldRender> {
        self.alert_ctx = Context::Danger(error);
        Ok(true)
    }

    pub(super) fn handle_post(&mut self) -> Result<ShouldRender> {
        debug!("Recipe {:?}", self.state);
        let request = Request::post("/ajax/recipe/")
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&self.state).map_err(anyhow::Error::from))
            .map_err(anyhow::Error::from)?;
        let task = self.fetch_svc.fetch(
            request,
            self.link.callback(
                move |response: Response<Text>| match response.into_parts() {
                    (meta, Ok(body)) if meta.status >= StatusCode::BAD_REQUEST => {
                        Msg::Failure(body)
                    }
                    (_, Ok(body)) => Msg::Posted(body),
                    (_, Err(error)) => {
                        error!("{}", error);
                        Msg::Failure(format!("{}", error))
                    }
                },
            ),
        )?;
        self.fetch_tsk = Some(task);
        Ok(false)
    }

    pub(super) fn handle_posted(&mut self, body: String) -> Result<ShouldRender> {
        self.validate()?;
        self.alert_ctx = Context::Success(body);
        self.fetch_tsk = None;
        Ok(true)
    }

    fn validate(&self) -> Result<()> {
        validate_url(&self.state.url)?;
        validate_payload(&self.state.payload)?;
        Ok(())
    }
}

fn validate_url(url: &str) -> Result<()> {
    if url.is_empty() {
        bail!("The url field is required!")
    }
    let uri = url
        .parse::<Uri>()
        .with_context(|| format!("Unable to parse, \"{}\", as a uri!", url))?;
    debug!("Parsed uri as {}", uri);
    trace!("Host {:?}", uri.host());
    if uri.host().is_none() {
        bail!("The url field, \"{}\", must contain a hostname!", url)
    }
    trace!("Scheme {:?}", uri.scheme());
    if uri.scheme().is_none() {
        bail!(
            "The url field, \"{}\", must start with a scheme, for example \"https://\"!",
            url
        )
    }
    trace!("Path and query {:?}", uri.path_and_query());
    if let Some(path_and_query) = uri.path_and_query() {
        if !path_and_query.as_str().starts_with("/api") {
            bail!("The url field, \"{}\", must start with \"/api\"!", url)
        }
    } else {
        bail!(
            "The url field, \"{}\", must include a path, starting with \"/api\"!",
            url
        )
    }
    Ok(())
}

fn validate_payload(payload: &str) -> Result<()> {
    serde_json::from_str::<serde_json::Value>(payload)
        .with_context(|| "The payload field must be valid JSON!".to_owned())?;
    Ok(())
}
