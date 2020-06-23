use super::{Msg, Rule, RuleType, Rules};
use yew::prelude::*;

impl Rules {
    // TODO re-factor out Rule component to manage input values locally
    pub(super) fn render_rule(&self, index: usize, r: &Rule) -> Html {
        html! {
            <li class="list-group-item">
                <div class="toolbar mb-3">
                    <button
                        type="button"
                        class="btn btn-secondary"
                        onclick=self.link.callback(move |_| Msg::RemoveRule(index))
                    >{ "Remove This Rule" }</button>
                </div>
                <div class="form-row mb-3">
                    <div class="col">
                        <label for="rule">{ "Rule Type" }</label>
                        <select
                            name="rule"
                            class="form-control"
                            onchange=self.link.callback(move |evt| Msg::RuleTypeChange(evt, index))
                        >
                            <option selected={r.rule_type.is_none()} disabled=true>{ "Choose Rule Type" }</option>
                            <option selected={r.rule_type == Some(RuleType::Authenticated)}>{ "Authenticated Call" }</option>
                            <option selected={r.rule_type == Some(RuleType::Subject)}>{ "With Subject" }</option>
                        </select>
                    </div>
                    {
                        match r.rule_type {
                            Some(RuleType::Authenticated) => self.render_key_path(r, index),
                            Some(RuleType::Subject) => self.render_subject(r, index),
                            _ => html! { <div class="col" /> }
                        }
                    }
                </div>
            </li>
        }
    }

    fn render_key_path(&self, r: &Rule, index: usize) -> Html {
        html! {
            <div class="col">
                <label for="subject">{ "Key Path" }</label>
                <input
                    name="key_path"
                    class="form-control"
                    type="text"
                    onchange=self.link.callback(move |evt| Msg::KeyPathChange(evt, index))
                    aria-describedby="key_path_help"
                    value=r.key_path.clone().unwrap_or_default()
                />
                <small id="key_path_help">
                    { "This rule will match if the authentication JWT can be verified with the
                    provided key. The value for the Key Path is the path relative to a directory
                        full of keys, for example \"/qa/my_service/public/sso/0\"." }
                </small>
            </div>
        }
    }

    fn render_subject(&self, r: &Rule, index: usize) -> Html {
        html! {
            <div class="col">
                <label for="subject">{ "Subject" }</label>
                <input
                    name="subject"
                    class="form-control"
                    type="text"
                    onchange=self.link.callback(move |evt| Msg::SubjectChange(evt, index))
                    aria-describedby="subject_help"
                    value=r.subject.clone().unwrap_or_default()
                />
                <small id="subject_help">{ "This rule will match the value of the subject claim in the authentication JWT." }</small>
            </div>
        }
    }
}
