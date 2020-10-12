use super::*;
use crate::components::editor::rule_editor;

impl KeyPathSelector {
    pub(super) fn view_selected(&self) -> Html {
        html! {
            <div class="input-group mb-3">
                <div class="input-group-prepend">
                    <label class="input-group-text" for="selected">{ self.label_txt() }</label>
                </div>
                <input
                    name="selected"
                    class=self.classes()
                    disabled=true value=self.state.selected.join(self.separator())
                />
                <div class="input-group-append">
                    <Button color=Color::Secondary on_click=self.link.callback(|_| Msg::KeyPathReset)>
                        { "Reset" }
                    </Button>
                </div>
                { rule_editor::render_validation_feedback(&self.props.errors, "invalid_authenticated_rule") }
            </div>
        }
    }

    pub(super) fn view_select(&self) -> Html {
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
        if self.state.complete {
            html! {}
        } else {
            html! {
                <div>
                    <label for="key_path">{ label_txt }</label>
                    <select
                        name=format!("key_path_{}", self.state.selected.len())
                        ref=self.select_ref.clone()
                        class="form-control"
                        onchange=self.link.callback(Msg::KeyPathChange)
                        aria_describedby="key_path_help"
                    >
                        <option
                            id=format!("prompt-{}", self.state.selected.len())
                            selected=true
                        >
                            { format!("Choose {}", prompt) }
                        </option>
                        { for self.state.candidates.iter().map(view_candidates) }
                    </select>
                </div>
            }
        }
    }

    pub(super) fn kind_help(&self) -> &str {
        if self.props.key_path_is_file {
            "The value for the Key Reference is the property path to a specific key within a JSON file, for example \"my_service.0\"."
        } else {
            "The value for the Key Path is the path relative to a directory full of keys, for example \"/qa/my_service/public/sso/0\"."
        }
    }

    pub(super) fn separator(&self) -> &str {
        if let shared::KeyPathKind::Directory { .. } = self.state.kind {
            "/"
        } else {
            "."
        }
    }

    fn label_txt(&self) -> &str {
        if self.props.key_path_is_file {
            "Selected Key Reference"
        } else {
            "Selected Key Path"
        }
    }

    fn classes(&self) -> Classes {
        let class = Classes::from("form-control");
        let class = class.extend(self.props.class.clone());
        class.extend(super::super::validation_class_for_rule(
            &self.props.errors,
            RuleType::Authenticated,
            &Some(RuleType::Authenticated),
            "invalid_authenticated_rule",
        ))
    }
}

fn view_candidates(c: &shared::KeyPathComponent) -> Html {
    html! {
        <option selected=false id=c.component.clone()>{ &c.component }</option>
    }
}
