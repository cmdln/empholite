use crate::HttpVerb;
use anyhow::{bail, Result};
use bootstrap_rs::prelude::*;
use yew::{prelude::*, web_sys::HtmlSelectElement};

pub(crate) struct VerbSelect {
    props: Props,
    link: ComponentLink<Self>,
    state: Option<HttpVerb>,
}

pub(crate) enum Msg {
    MethodChange(ChangeData),
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub(crate) struct Props {
    #[prop_or_default]
    pub(crate) verb: Option<HttpVerb>,
    pub(crate) on_change: Callback<HttpVerb>,
    pub(crate) on_error: Callback<String>,
}

impl Component for VerbSelect {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = props.verb.clone();
        Self { props, link, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let result = if let Msg::MethodChange(ChangeData::Select(selected)) = msg {
            self.handle_method(selected)
        } else {
            Ok(false)
        };
        match result {
            Ok(true) => {
                if let Some(http_verb) = self.state.as_ref() {
                    self.props.on_change.emit(http_verb.clone());
                } else {
                    self.props
                        .on_error
                        .emit("Invalid selection for HTTP method!".to_owned());
                }
                true
            }
            Ok(false) => false,
            Err(error) => {
                self.props.on_error.emit(error.to_string());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        render_on_change(&mut self.props, props)
    }

    fn view(&self) -> Html {
        use HttpVerb::*;
        html! {
            <>
                <label for="verb">{ "HTTP Verbs" }</label>
                <select
                    class="form-control"
                    name="verb"
                    onchange=self.link.callback(move |evt| Msg::MethodChange(evt))
                >
                    <option selected={self.state.is_none()} disabled=true>{ "Choose HTTP Verb" }</option>
                    <option selected={self.state == Some(Get)}>{ "Get" }</option>
                    <option selected={self.state == Some(Post)}>{ "Post" }</option>
                    <option selected={self.state == Some(Put)}>{ "Put" }</option>
                    <option selected={self.state == Some(Delete)}>{ "Delete" }</option>
                </select>
            </>
        }
    }
}

impl VerbSelect {
    fn handle_method(&mut self, selected: HtmlSelectElement) -> Result<ShouldRender> {
        use HttpVerb::*;
        self.state = Some(match selected.selected_index() {
            1 => Get,
            2 => Post,
            3 => Put,
            4 => Delete,
            _ => bail!("Invalid selection for HTTP method!"),
        });
        Ok(true)
    }
}
