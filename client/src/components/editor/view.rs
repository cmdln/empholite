use super::{Editor, Msg};
use bootstrap_rs::{input::InputType, CardBody, CardHeader, CardText, FormGroup, Input, TextArea};
use yew::prelude::*;

impl Editor {
    pub(super) fn view_edit_body(&self) -> Html {
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

    pub(super) fn view_view_body(&self) -> Html {
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
