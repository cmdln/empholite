mod actions;
mod key_path;
mod verb_select;
mod view;

use crate::{prelude::*, HttpVerb, Rule, RuleType};
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
            Err(error) => {
                self.props.on_error.emit(format!("{}", error));
                false
            }
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

fn render_validation_feedback(
    errors: &Option<Option<Box<ValidationErrors>>>,
    code: &'static str,
) -> Html {
    match errors.as_ref() {
        Some(Some(errors)) => {
            let errors = errors.field_errors();
            if let Some(errors) = errors.get("__all__") {
                html! {
                    <div class="invalid-feedback">
                        { for errors.iter().filter(|error| error.code == code).filter_map(|error| error.message.as_ref()) }
                    </div>
                }
            } else {
                html! {}
            }
        }
        Some(None) => {
            html! {}
        }
        None => html! {},
    }
}

fn validation_class_for_rule(
    errors: &Option<Option<Box<ValidationErrors>>>,
    for_rule_type: RuleType,
    selected_rule_type: &Option<RuleType>,
    code: &str,
) -> Classes {
    if selected_rule_type
        .as_ref()
        .map(|selected| selected == &for_rule_type)
        .unwrap_or_default()
    {
        match errors.as_ref() {
            Some(Some(errors)) if invalid_for(errors, code) => Classes::from("is-invalid"),
            Some(Some(_)) | Some(None) => Classes::from("is-valid"),
            None => Classes::new(),
        }
    } else {
        Classes::new()
    }
}

fn invalid_for(errors: &ValidationErrors, code: &str) -> bool {
    let errors = errors.field_errors();
    errors
        .get("__all__")
        .map(|errors| errors.iter().any(|error| error.code == code))
        .unwrap_or_default()
}
