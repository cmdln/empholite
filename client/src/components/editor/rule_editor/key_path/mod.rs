mod actions;
mod view;

use crate::RuleType;
use anyhow::{format_err, Result};
use bootstrap_rs::{prelude::*, Button};
use validator::ValidationErrors;
use yew::{prelude::*, services::fetch::FetchTask};

pub(crate) struct KeyPathSelector {
    link: ComponentLink<Self>,
    fetch_tsk: Option<FetchTask>,
    state: KeyPathCompletions,
    props: Props,
    select_ref: NodeRef,
}

struct KeyPathCompletions {
    candidates: Vec<shared::KeyPathComponent>,
    selected: Vec<String>,
    kind: shared::KeyPathKind,
    complete: bool,
}

impl From<shared::KeyPathCompletions> for KeyPathCompletions {
    fn from(k: shared::KeyPathCompletions) -> Self {
        let shared::KeyPathCompletions {
            candidates,
            selected,
            kind,
        } = k;
        let complete = false;
        Self {
            candidates,
            selected,
            kind,
            complete,
        }
    }
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub(crate) struct Props {
    #[prop_or_default]
    pub(crate) name: Option<String>,
    #[prop_or_default]
    pub(crate) value: Option<String>,
    pub(crate) key_path_is_file: bool,
    #[prop_or_default]
    pub(crate) class: Classes,
    pub(crate) on_change: Callback<String>,
    pub(crate) on_error: Callback<String>,
    #[prop_or_default]
    pub(crate) errors: Option<Option<Box<ValidationErrors>>>,
    #[prop_or_default]
    pub(crate) aria_describedby: Option<String>,
}

pub(crate) enum Msg {
    Fetch,
    Fetched(String),
    KeyPathChange(ChangeData),
    KeyPathReset,
    Failure(String),
}

impl Component for KeyPathSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        let complete = props.value.as_ref().is_some();
        if !complete {
            link.send_message(Msg::Fetch);
        }
        let fetch_tsk = None;
        let state = KeyPathCompletions {
            candidates: Vec::new(),
            selected: props
                .value
                .as_deref()
                .map(into_selected)
                .unwrap_or_default(),
            kind: shared::KeyPathKind::Directory,
            complete,
        };
        let select_ref = NodeRef::default();
        Self {
            link,
            fetch_tsk,
            state,
            props,
            select_ref,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        let result: Result<bool> = match msg {
            Fetch => self.handle_fetch(),
            Fetched(body) => self.handle_fetched(body),
            KeyPathChange(ChangeData::Select(selected)) => self.handle_key_change(selected),
            KeyPathChange(_) => Ok(false),
            KeyPathReset => self.handle_key_reset(),
            Failure(error) => Err(format_err!("{}", error)),
        };
        match result {
            Ok(should_render) => should_render,
            Err(error) => {
                self.props.on_error.emit(format!("{}", error));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        render_on_change(&mut self.props, props)
    }

    fn view(&self) -> Html {
        html! {
            <div class="col">
                { self.view_selected() }
                { self.view_select() }
                <small id="key_path_help">
                    { format!("This rule will match if the authentication JWT can be verified with the
                    provided key. {}", self.kind_help()) }
                </small>
            </div>
        }
    }
}

fn into_selected(s: &str) -> Vec<String> {
    let separator = if s.contains('/') { '/' } else { '.' };
    s.split(separator).map(ToOwned::to_owned).collect()
}
