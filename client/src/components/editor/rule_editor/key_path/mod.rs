mod actions;

use crate::RuleType;
use anyhow::Result;
use validator::ValidationErrors;
use yew::{prelude::*, services::fetch::FetchTask};

pub(crate) struct KeyPathSelector {
    link: ComponentLink<Self>,
    fetch_tsk: Option<FetchTask>,
    state: shared::KeyPathCompletions,
    props: Props,
}

#[derive(Properties, Debug, Clone)]
pub(crate) struct Props {
    #[prop_or_default]
    pub(crate) name: Option<String>,
    #[prop_or_default]
    pub(crate) value: Option<String>,
    pub(crate) key_path_is_file: bool,
    #[prop_or_default]
    pub(crate) class: Classes,
    pub(crate) on_change: Callback<String>,
    #[prop_or_default]
    pub(crate) errors: Option<Option<Box<ValidationErrors>>>,
    #[prop_or_default]
    pub(crate) aria_describedby: Option<String>,
}

pub(crate) enum Msg {
    Fetch,
    Fetched(String),
    KeyPathChange(ChangeData),
    Failure(String),
}

impl Component for KeyPathSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::Fetch);
        let fetch_tsk = None;
        let state = shared::KeyPathCompletions {
            candidates: Vec::new(),
            selected: Vec::new(),
            kind: shared::KeyPathKind::Directory,
        };
        Self {
            link,
            fetch_tsk,
            state,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        let result: Result<bool> = match msg {
            Fetch => self.handle_fetch(),
            Fetched(body) => self.handle_fetched(body),
            KeyPathChange(ChangeData::Select(selected)) => self.handle_key_change(selected),
            _ => Ok(false),
        };
        match result {
            Ok(true) => {
                // TODO add change handler prop
                // self.props.on_change.emit(self.state.clone());
                true
            }
            Ok(false) => false,
            // TODO emit error
            Err(_) => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let kind_help = if self.props.key_path_is_file {
            "The value for the Key Reference is the property path to a specific key within a JSON file, for example \"my_service.0\"."
        } else {
            "The value for the Key Path is the path relative to a directory full of keys, for example \"/qa/my_service/public/sso/0\"."
        };
        let label_txt = if self.props.key_path_is_file {
            "Key Reference"
        } else {
            "Key Path"
        };
        let prompt = if self.props.key_path_is_file {
            "next property for key reference"
        } else {
            "next component for key path"
        };
        let class = Classes::from("form-control");
        let class = class.extend(self.props.class.clone());
        let class = class.extend(super::validation_class_for_rule(
            &self.props.errors,
            RuleType::Authenticated,
            &Some(RuleType::Authenticated),
            "invalid_authenticated_rule",
        ));
        html! {
            <div class="col">
                <label for="key_path">{ label_txt }</label>
                <select
                    name="key_path"
                    class=class
                    onchange=self.link.callback(Msg::KeyPathChange)
                    aria_describedby="key_path_help"
                >
                    <option selected=true disabled=true>{ format!("Choose {}", prompt) }</option>
                    { for self.state.candidates.iter().map(view_candidates) }
                </select>
                <small id="key_path_help">
                    { format!("This rule will match if the authentication JWT can be verified with the
                    provided key. {}", kind_help) }
                </small>
                { super::render_validation_feedback(&self.props.errors, "invalid_authenticated_rule") }
            </div>
        }
    }
}

fn view_candidates(c: &shared::KeyPathComponent) -> Html {
    html! {
        <option>{ &c.component }</option>
    }
}
