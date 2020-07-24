use super::{rule_editor::RuleEditor, Editor, Mode, Msg};
use crate::{AppRoute, Rule};
use bootstrap_rs::{
    input::InputType, Breadcrumb, BreadcrumbItem, ButtonGroup, CardBody, CardHeader, CardText,
    FormGroup, Input, TextArea,
};
use validator::{ValidationErrors, ValidationErrorsKind};
use yew::prelude::*;
use yew_router::prelude::*;

impl Editor {
    pub(super) fn render_breadcrumbs(&self) -> Html {
        html! {
            <Breadcrumb>
                <BreadcrumbItem active=false>
                    <RouterAnchor<AppRoute> route=AppRoute::Index>
                    { "Recipe Management" }
                    </RouterAnchor<AppRoute>>
                </BreadcrumbItem>
                <BreadcrumbItem active=true>
                {
                    match (self.state.id.is_some(), &self.mode) {
                        (true, Mode::View) => "View Recipe",
                        (true, Mode::Edit) => "Edit Recipe",
                        (false, _) => "Create New Recipe",
                    }
                }
                </BreadcrumbItem>
            </Breadcrumb>
        }
    }

    pub(super) fn render_toolbar(&self) -> Html {
        html! {
            <div
                class="btn-toolbar mb-3"
                role="toolbar"
                aria-label="Toolbar"
            >
            {
                if self.mode == Mode::View {
                    self.render_view_toolbar()
                } else {
                    self.render_edit_toolbar()
                }
            }
            </div>
        }
    }

    pub(super) fn render_body(&self) -> Html {
        if self.mode == Mode::View {
            self.render_view_body()
        } else {
            self.render_edit_body()
        }
    }

    fn render_edit_toolbar(&self) -> Html {
        html! {
            <ButtonGroup>
                <button
                    type="button" onclick=self.link.callback(|_| Msg::Post)
                    class="btn btn-primary"
                >
                    { "Save" }
                </button>
                {
                    if self.props.id.is_some() {
                        self.render_cancel_edit()
                    } else {
                        self.render_cancel_add()
                    }
                }
            </ButtonGroup>
        }
    }

    fn render_cancel_edit(&self) -> Html {
        html! {
            <button
                type="button" onclick=self.link.callback(|_| Msg::Cancel)
                class="btn btn-secondary"
            >
                { "Cancel" }
            </button>
        }
    }

    fn render_cancel_add(&self) -> Html {
        html! {
            <RouterButton<AppRoute> route=AppRoute::Index classes="btn btn-secondary">
                { "Cancel" }
            </RouterButton<AppRoute>>
        }
    }

    fn render_view_toolbar(&self) -> Html {
        html! {
                <button
                    type="button" onclick=self.link.callback(|_| Msg::Edit)
                    class="btn btn-primary"
                >
                    { "Edit" }
                </button>
        }
    }

    fn render_edit_body(&self) -> Html {
        html! {
            <CardBody>
                <FormGroup>
                    <label for="url">
                        { "Endpoint" }
                    </label>
                    <Input
                        id="url"
                        input_type=InputType::Text
                        value=self.state.url.clone()
                        on_change=self.link.callback(|value| Msg::UrlChanged(value))
                        valid=is_valid("url", &self.errors)
                    />
                    { render_validation_feedback("url", &self.errors) }
                </FormGroup>
                <FormGroup>
                    <p>{ "Rules" }</p>
                    {
                        if self.state.rules.is_empty() {
                            html! {}
                        } else {
                            html! {
                                <ol class="list-group mb-3">
                                    { for self.state.rules.iter().enumerate().map(|(index, r)| self.render_edit_rule(r, index)) }
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
                <FormGroup>
                    <label for="payload">
                        { "Payload" }
                    </label>
                    <TextArea
                        name="payload"
                        on_change=self.link.callback(|value| Msg::PayloadChanged(value))
                        valid=is_valid("payload", &self.errors)
                    >
                        { self.state.payload.clone() }
                    </TextArea>
                    { render_validation_feedback("payload", &self.errors) }
                </FormGroup>
            </CardBody>
        }
    }

    fn render_view_body(&self) -> Html {
        html! {
            <>
                <CardHeader>
                    { self.state.url.clone() }
                </CardHeader>
                <CardBody>
                    <CardText>{ "Rules" }</CardText>
                    <ol class="list-group mb-3">
                        { for self.state.rules.iter().map(render_view_rule) }
                    </ol>
                    <CardText>
                        { "Payload" }
                    </CardText>
                    <TextArea readonly=true>
                        { self.state.payload.clone() }
                    </TextArea>
                </CardBody>
            </>
        }
    }

    fn render_edit_rule(&self, r: &Rule, index: usize) -> Html {
        let errors = self
            .errors
            .as_ref()
            .and_then(|errors| errors.errors().get("rules"))
            .map(|errors| {
                if let ValidationErrorsKind::List(errors) = errors {
                    errors.get(&index).map(ToOwned::to_owned)
                } else {
                    None
                }
            });
        html! {
            <RuleEditor
                rule=r.to_owned()
                key_path_is_file=shared::KeyPathKind::File == self.config.key_path_kind
                on_change=self.link.callback(move |rule| Msg::RuleChanged(rule, index))
                on_remove=self.link.callback(move |_| Msg::RemoveRule(index))
                errors=errors
            />
        }
    }
}

fn render_view_rule(r: &Rule) -> Html {
    use crate::RuleType::*;
    html! {
        <li class="list-group-item">
        {
            match r.rule_type {
                Some(Authenticated) => html! {
                    <>
                        { "Authenticated by verifying JWT with public key, " }
                        { r.key_path.clone().unwrap_or_default() }
                    </>
                },
                Some(Subject) => html! {
                    <>
                        { "Subject claim of authentication JWT matches value, " }
                        { r.subject.clone().unwrap_or_default() }
                    </>
                },
                _ => html! {}
            }
        }
        </li>
    }
}

fn render_validation_feedback(field: &'static str, errors: &Option<ValidationErrors>) -> Html {
    if let Some(ref errors) = errors {
        let errors = errors.field_errors();
        if let Some(errors) = errors.get(field) {
            html! {
                <div class="invalid-feedback">
                    { for errors.iter().filter_map(|error| error.message.as_ref()) }
                </div>
            }
        } else {
            html! {}
        }
    } else {
        html! {}
    }
}

fn is_valid(field: &'static str, errors: &Option<ValidationErrors>) -> Option<bool> {
    errors
        .as_ref()
        .map(|errors| !errors.field_errors().contains_key(field))
}
