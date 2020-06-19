use super::{Editor, Mode, Msg};
use crate::AppRoute;
use bootstrap_rs::{
    input::InputType, Breadcrumb, BreadcrumbItem, ButtonGroup, CardBody, CardHeader, CardText,
    FormGroup, Input, TextArea,
};
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
                <button
                    type="button" onclick=self.link.callback(|_| Msg::Cancel)
                    class="btn btn-secondary"
                >
                    { "Cancel" }
                </button>
            </ButtonGroup>
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
                    <Input id="url" input_type=InputType::Text value=self.state.url.clone() on_change=self.link.callback(|value| Msg::UrlChanged(value))/>
                </FormGroup>
                <FormGroup>
                    <label for="payload">
                        { "Payload" }
                    </label>
                    <TextArea
                        name="payload"
                        value=self.state.payload.clone()
                        on_change=self.link.callback(|value| Msg::PayloadChanged(value))
                    />
                </FormGroup>
                <div class="mt-3">
                    <button
                        class="btn btn-primary"
                        type="button"
                        onclick=self.link.callback(|_| Msg::Post)
                    >
                        { "Save" }
                    </button>
                </div>
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
                    <CardText>
                        { self.state.payload.clone() }
                    </CardText>
                </CardBody>
            </>
        }
    }
}
