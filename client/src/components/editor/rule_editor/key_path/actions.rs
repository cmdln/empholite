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
        self.state = serde_json::from_str::<shared::KeyPathCompletions>(&body)?.into();
        if self.state.candidates.len() == 1 && self.state.candidates[0].leaf {
            self.state
                .selected
                .push(self.state.candidates[0].component.clone());
            self.complete();
        } else if let Some(select) = self.select_ref.cast::<HtmlSelectElement>() {
            select.set_selected_index(0);
        }
        self.fetch_tsk = None;
        Ok(true)
    }

    pub(super) fn handle_key_reset(&mut self) -> Result<ShouldRender> {
        self.state.selected = Vec::new();
        self.state.complete = false;
        self.link.send_message(Msg::Fetch);
        self.props.on_change.emit(String::default());
        Ok(true)
    }

    pub(super) fn handle_key_change(
        &mut self,
        selected: HtmlSelectElement,
    ) -> Result<ShouldRender> {
        if selected.selected_index() != 0 {
            let selected = (selected.selected_index() - 1) as usize;
            let selected = &self.state.candidates[selected];
            self.state.selected.push(selected.component.clone());
            if selected.leaf {
                self.complete();
            } else {
                self.link.send_message(Msg::Fetch);
            }
        }
        Ok(true)
    }

    fn complete(&mut self) {
        self.state.complete = true;
        self.props
            .on_change
            .emit(self.state.selected.join(self.separator()));
    }
}
