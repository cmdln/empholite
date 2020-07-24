use super::{Msg, RuleEditor};
use crate::RuleType;
use bootstrap_rs::{input::InputType, prelude::*, Button, ButtonToolbar, Input};
use validator::ValidationErrors;
use yew::prelude::*;

impl RuleEditor {
    pub(super) fn render_editor(&self) -> Html {
        html! {
            <li class="list-group-item">
                <ButtonToolbar margin=Margin(Edge::Bottom, 3)>
                    <Button
                        on_click=self.link.callback(|_| Msg::Remove)
                        color=Color::Secondary
                    >
                        { "Remove This Rule " }
                    </Button>
                </ButtonToolbar>
                <div class=validation_parent_class(&self.props.errors, "form-row mb-3")>
                    <div class="col">
                        <label for="rule">{ "Rule Type" }</label>
                        <select
                            name="rule"
                            class=validation_class(&self.props.errors, "rule_type_required", "form-control")
                            onchange=self.link.callback(move |evt| Msg::TypeChange(evt))
                        >
                            <option selected={self.state.rule_type.is_none()} disabled=true>{ "Choose Rule Type" }</option>
                            <option selected={self.state.rule_type == Some(RuleType::Authenticated)}>{ "Authenticated Call" }</option>
                            <option selected={self.state.rule_type == Some(RuleType::Subject)}>{ "With Subject" }</option>
                        </select>
                        { self.render_validation_feedback("rule_type_required") }
                    </div>
                    {
                        match self.state.rule_type {
                            Some(RuleType::Authenticated) => self.render_key_path(),
                            Some(RuleType::Subject) => self.render_subject(),
                            _ => html! { <div class="col" /> }
                        }
                    }
                </div>
            </li>
        }
    }

    fn render_key_path(&self) -> Html {
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
        html! {
            <div class="col">
                <label for="subject">{ label_txt }</label>
                <Input
                    name="key_path"
                    class=validation_class_for_rule(&self.props.errors, RuleType::Authenticated, &self.state.rule_type, "invalid_authenticated_rule", )
                    input_type=InputType::Text
                    on_change=self.link.callback(move |value| Msg::KeyPathChange(value))
                    aria_describedby="key_path_help"
                    value=self.state.key_path.clone().unwrap_or_default()
                />
                <small id="key_path_help">
                    { format!("This rule will match if the authentication JWT can be verified with the
                    provided key. {}", kind_help) }
                </small>
                { self.render_validation_feedback("invalid_authenticated_rule") }
            </div>
        }
    }

    fn render_subject(&self) -> Html {
        html! {
            <div class="col">
                <label for="subject">{ "Subject" }</label>
                <Input
                    name="subject"
                    class=validation_class_for_rule(&self.props.errors, RuleType::Subject, &self.state.rule_type, "invalid_subject_rule")
                    input_type=InputType::Text
                    on_change=self.link.callback(move |value| Msg::SubjectChange(value))
                    aria_describedby="subject_help"
                    value=self.state.subject.clone().unwrap_or_default()
                />
                <small id="subject_help">{ "This rule will match the value of the subject claim in the authentication JWT." }</small>
                { self.render_validation_feedback("invalid_subject_rule") }
            </div>
        }
    }

    fn render_validation_feedback(&self, code: &'static str) -> Html {
        match self.props.errors.as_ref() {
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
}

fn validation_parent_class(
    errors: &Option<Option<Box<ValidationErrors>>>,
    prefix: &str,
) -> Classes {
    let mut class = Classes::from(prefix);
    match errors.as_ref() {
        Some(Some(_)) => {
            class.push("was-invalidated");
        }
        Some(None) => {
            class.push("was-validated");
        }
        None => {}
    }
    class
}

fn validation_class(
    errors: &Option<Option<Box<ValidationErrors>>>,
    code: &str,
    prefix: &str,
) -> Classes {
    let mut class = Classes::from(prefix);
    match errors.as_ref() {
        Some(Some(errors)) if invalid_for(errors, code) => {
            class.push("is-invalid");
        }
        Some(Some(_)) | Some(None) => {
            class.push("is-valid");
        }
        None => {}
    }
    class
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
