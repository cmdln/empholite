use super::*;
use log::error;
use yew::{
    format::{Nothing, Text},
    services::{
        fetch::{Request, Response, StatusCode},
        FetchService,
    },
    web_sys::HtmlSelectElement,
};

impl KeyPathSelector {
    pub(super) fn handle_fetch(&mut self) -> Result<ShouldRender> {
        let request = Request::get(format!("/ajax/key_path/{}", self.state.selected.join("/")))
            .body(Nothing)
            .map_err(anyhow::Error::from)?;
        let task = FetchService::fetch(
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
        self.state = serde_json::from_str(&body)?;
        self.fetch_tsk = None;
        Ok(true)
    }

    pub(super) fn handle_key_change(
        &mut self,
        selected: HtmlSelectElement,
    ) -> Result<ShouldRender> {
        use log::debug;
        let selected = (selected.selected_index() - 1) as usize;
        debug!("Selected component, {}", selected);
        let selected = self.state.candidates[selected].component.clone();
        self.state.selected.push(selected.clone());
        self.props.on_change.emit(selected);
        debug!("Issuing a new fetch");
        self.link.send_message(Msg::Fetch);
        Ok(true)
    }
}
