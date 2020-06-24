use super::{types::Mode, Editor, Msg};
use crate::{components::alert::Context, Rule};
use anyhow::{format_err, Context as _, Result};
use log::{debug, error};
use validator::Validate;
use yew::{
    format::{Nothing, Text},
    prelude::*,
    services::fetch::{Request, Response, StatusCode},
};

impl Editor {
    pub(super) fn handle_edit(&mut self) -> Result<ShouldRender> {
        self.mode = Mode::Edit;
        Ok(true)
    }

    pub(super) fn handle_cancel(&mut self) -> Result<ShouldRender> {
        self.mode = Mode::View;
        self.link.send_message(Msg::Fetch);
        Ok(false)
    }

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
        self.state = state.into();
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

    pub(super) fn handle_add_rule(&mut self) -> Result<ShouldRender> {
        self.state.rules.push(Rule::default());
        Ok(true)
    }

    pub(super) fn handle_rule_changed(&mut self, rule: Rule, index: usize) -> Result<ShouldRender> {
        self.state.rules[index] = rule;
        Ok(true)
    }

    pub(super) fn handle_remove_rule(&mut self, index: usize) -> Result<ShouldRender> {
        self.state.rules.remove(index);
        Ok(true)
    }

    pub(super) fn handle_post(&mut self) -> Result<ShouldRender> {
        if let Err(errors) = self.state.validate() {
            error!("Validation errors {:?}", errors);
            self.errors = Some(errors);
            Ok(true)
        } else {
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
    }

    pub(super) fn handle_posted(&mut self, body: String) -> Result<ShouldRender> {
        let state: shared::Recipe = serde_json::from_str(&body)?;
        self.state = state.into();
        self.alert_ctx = Context::Success("Saved!".into());
        self.mode = Mode::View;
        self.fetch_tsk = None;
        Ok(true)
    }
}
