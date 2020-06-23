mod view;

use crate::prelude::*;
use crate::{Rule, RuleType};
use bootstrap_rs::FormGroup;
use yew::prelude::*;

pub(crate) struct Rules {
    link: ComponentLink<Self>,
    rules: Vec<Rule>,
}

pub(crate) enum Msg {
    AddRule,
    RemoveRule(usize),
    RuleTypeChange(ChangeData, usize),
    SubjectChange(ChangeData, usize),
    KeyPathChange(ChangeData, usize),
    // TODO add messages for add, remove and change to prop to parent
}

// TODO create props for initial passing of rules vec
// TODO add call back props to props for add, remove, change rules

impl Component for Rules {
    type Properties = ();
    type Message = Msg;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            rules: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        let result = match msg {
            AddRule => {
                self.rules.push(Rule::default());
                Ok(true)
            }
            RemoveRule(index) => {
                self.rules.remove(index);
                Ok(true)
            }
            RuleTypeChange(ChangeData::Select(selected), index) => {
                let rule = &mut self.rules[index];
                rule.rule_type = match selected.selected_index() {
                    1 => Some(RuleType::Authenticated),
                    2 => Some(RuleType::Subject),
                    _ => None,
                };
                Ok(true)
            }
            SubjectChange(ChangeData::Value(subject), index) => {
                render_on_assign(&mut self.rules[index].subject, Some(subject))
            }
            KeyPathChange(ChangeData::Value(key_path), index) => {
                render_on_assign(&mut self.rules[index].key_path, Some(key_path))
            }
            _ => Ok(false),
        };
        match result {
            Ok(should_render) => should_render,
            // TODO prop error to parent
            Err(_error) => true,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        // TODO override local rules with those from parent
        false
    }

    fn view(&self) -> Html {
        html! {
            <FormGroup>
                <p>{ "Rules" }</p>
                {
                    if self.rules.is_empty() {
                        html! {}
                    } else {
                        html! {
                            <ol class="list-group mb-3">
                                { for self.rules.iter().enumerate().map(|(index, r)| self.render_rule(index, r)) }
                            </ol>
                        }
                    }
                }
                <button
                    type="button"
                    class="btn btn-secondary"
                    onclick=self.link.callback(|_| Msg::AddRule)
                >
                    { "Add New Rule" }
                </button>
            </FormGroup>
        }
    }
}
