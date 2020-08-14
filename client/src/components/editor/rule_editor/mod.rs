mod actions;
mod verb_select;
mod view;

use crate::{prelude::*, HttpVerb, Rule};
use bootstrap_rs::prelude::*;
use validator::ValidationErrors;
use yew::prelude::*;

pub(super) struct RuleEditor {
    link: ComponentLink<Self>,
    props: Props,
    state: Rule,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub(super) struct Props {
    pub(super) rule: Rule,
    pub(super) key_path_is_file: bool,
    pub(super) on_change: Callback<Rule>,
    pub(super) on_error: Callback<String>,
    pub(super) on_remove: Callback<()>,
    #[prop_or_default]
    pub(super) errors: Option<Option<Box<ValidationErrors>>>,
}

pub(super) enum Msg {
    TypeChange(ChangeData),
    KeyPathChange(String),
    SubjectChange(String),
    HttpMethodChange(HttpVerb),
    Remove,
    Failure(String),
}

impl Component for RuleEditor {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = props.rule.clone();
        Self { link, props, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        let result = match msg {
            TypeChange(ChangeData::Select(selected)) => self.handle_type(selected),
            SubjectChange(subject) => {
                opt_render_on_assign(&mut self.state.subject, InputString(subject))
            }
            KeyPathChange(key_path) => {
                opt_render_on_assign(&mut self.state.key_path, InputString(key_path))
            }
            HttpMethodChange(http_verb) => {
                opt_render_on_assign(&mut self.state.http_method, http_verb)
            }
            TypeChange(_) => Ok(false),
            Remove => self.handle_remove(),
            Failure(error) => {
                self.props.on_error.emit(error);
                Ok(true)
            }
        };
        match result {
            Ok(true) => {
                self.props.on_change.emit(self.state.clone());
                true
            }
            Ok(false) => false,
            // TODO emit error
            Err(_) => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let state = props.rule.clone();

        if self.props == props && self.state == state {
            false
        } else {
            render_on_assign(&mut self.state, state).unwrap_or_default()
                || render_on_change(&mut self.props, props)
        }
    }

    fn view(&self) -> Html {
        self.render_editor()
    }
}
