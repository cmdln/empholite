mod actions;
mod view;

use crate::{prelude::*, Rule};
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
    pub(super) on_remove: Callback<()>,
    #[prop_or_default]
    pub(super) errors: Option<Option<Box<ValidationErrors>>>,
}

pub(super) enum Msg {
    TypeChange(ChangeData),
    KeyPathChange(String),
    SubjectChange(String),
    Remove,
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
            SubjectChange(subject) => opt_render_on_assign(&mut self.state.subject, subject),
            KeyPathChange(key_path) => opt_render_on_assign(&mut self.state.key_path, key_path),
            TypeChange(_) => Ok(false),
            Remove => self.handle_remove(),
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
