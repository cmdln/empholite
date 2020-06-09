mod actions;

use crate::components::{alert::Context, Alert};
use bootstrap_rs::{
    input::InputType, prelude::*, Card, CardBody, Container, Input, Jumbotron, TextArea,
};
use log::debug;
use shared::Recipe;
use yew::{
    prelude::*,
    services::{fetch::FetchTask, FetchService},
};

pub(crate) struct Editor {
    link: ComponentLink<Self>,
    fetch_svc: FetchService,
    fetch_tsk: Option<FetchTask>,
    state: Recipe,
    alert_ctx: Context,
}

pub(crate) enum Msg {
    UrlChanged(String),
    RecipeChanged(String),
    Post,
    Posted(String),
    Failure(String),
}

impl Component for Editor {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fetch_svc = FetchService::new();
        let fetch_tsk = None;
        let state = Recipe::default();
        let alert_ctx = Context::default();
        Self {
            link,
            fetch_svc,
            fetch_tsk,
            state,
            alert_ctx,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let result = match msg {
            Self::Message::Post => self.handle_post(),
            Self::Message::Posted(body) => self.handle_posted(body),
            Self::Message::UrlChanged(url) => {
                debug!("Url changed, {}", url);
                self.state.url = url;
                Ok(true)
            }
            Self::Message::RecipeChanged(payload) => {
                self.state.payload = payload;
                Ok(true)
            }
            Self::Message::Failure(error) => {
                self.alert_ctx = Context::Danger(error);
                Ok(true)
            }
        };
        match result {
            Ok(should_render) => should_render,
            Err(error) => {
                self.alert_ctx = Context::Danger(format!("{}", error));
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container>
                <Jumbotron margin=Margin(Edge::Bottom, 3)>
                    <h1>{ "Empholite" }</h1>
                </Jumbotron>
                <Alert context=self.alert_ctx.clone() />
                <Card border=Border(Edge::All, Color::Primary)>
                    <CardBody>
                        <div class="form-group">
                            <label for="url">
                                { "Endpoint" }
                            </label>
                            <Input id="url" input_type=InputType::Text value=self.state.url.clone() on_change=self.link.callback(|value| Msg::UrlChanged(value))/>
                        </div>
                        <div class="form-group">
                            <label for="payload">
                                { "Payload" }
                            </label>
                            <TextArea
                                name="payload"
                                value=self.state.payload.clone()
                                on_change=self.link.callback(|value| Msg::RecipeChanged(value))
                            />
                        </div>
                        <div class="mt-3">
                            <button
                                class="btn btn-primary"
                                type="button"
                                onclick=self.link.callback(|_| Msg::Post)
                            >{ "Save" }</button>
                        </div>
                    </CardBody>
                </Card>
            </Container>
        }
    }
}
