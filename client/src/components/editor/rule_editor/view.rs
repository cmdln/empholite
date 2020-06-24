use super::{Msg, RuleEditor};
use crate::RuleType;
use bootstrap_rs::{input::InputType, prelude::*, Button, ButtonToolbar, Input};
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
                <div class="form-row mb-3">
                    <div class="col">
                        <label for="rule">{ "Rule Type" }</label>
                        <select
                            name="rule"
                            class="form-control"
                            onchange=self.link.callback(move |evt| Msg::TypeChange(evt))
                        >
                            <option selected={self.state.rule_type.is_none()} disabled=true>{ "Choose Rule Type" }</option>
                            <option selected={self.state.rule_type == Some(RuleType::Authenticated)}>{ "Authenticated Call" }</option>
                            <option selected={self.state.rule_type == Some(RuleType::Subject)}>{ "With Subject" }</option>
                        </select>
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
        html! {
            <div class="col">
                <label for="subject">{ "Key Path" }</label>
                <Input
                    name="key_path"
                    input_type=InputType::Text
                    on_change=self.link.callback(move |value| Msg::KeyPathChange(value))
                    aria_describedby="key_path_help"
                    value=self.state.key_path.clone().unwrap_or_default()
                />
                <small id="key_path_help">
                    { "This rule will match if the authentication JWT can be verified with the
                    provided key. The value for the Key Path is the path relative to a directory
                        full of keys, for example \"/qa/my_service/public/sso/0\"." }
                </small>
            </div>
        }
    }

    fn render_subject(&self) -> Html {
        html! {
            <div class="col">
                <label for="subject">{ "Subject" }</label>
                <Input
                    name="subject"
                    input_type=InputType::Text
                    on_change=self.link.callback(move |value| Msg::SubjectChange(value))
                    aria_describedby="subject_help"
                    value=self.state.subject.clone().unwrap_or_default()
                />
                <small id="subject_help">{ "This rule will match the value of the subject claim in the authentication JWT." }</small>
            </div>
        }
    }
}
