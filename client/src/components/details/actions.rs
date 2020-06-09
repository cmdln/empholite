use super::{Home, Msg};
use anyhow::Result;
use log::{debug, error};
use yew::{
    format::Text,
    prelude::*,
    services::fetch::{Request, Response, StatusCode},
};

impl Home {
    pub(super) fn handle_fetch(&mut self) -> Result<ShouldRender> {
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
        self.message = body;
        self.fetch_tsk = None;
        Ok(true)
    }
}
